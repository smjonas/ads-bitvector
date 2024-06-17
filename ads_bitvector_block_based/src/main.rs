use std::fs;
use std::mem::size_of;
use std::time::Instant;

const BLOCK_SIZE: usize = 512;

struct AuxiliaryTables {
    rank_0_table: Vec<usize>,
    rank_1_table: Vec<usize>,
    select_0_table: Vec<usize>,
    select_1_table: Vec<usize>,
}

fn main() {
    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let input_file_path = &args[1];
    let output_file_path = &args[2];
    let contents = std::fs::read_to_string(input_file_path).expect("Failed to read input file");
    // Start measurement after file has been read
    let now = Instant::now();
    let mut split_contents = contents.split("\n");
    let query_count = split_contents
        .next()
        .expect("Missing number of queries in input file")
        .parse()
        .expect("n is not an integer");
    let bits = split_contents
        .next()
        .expect("Missing bit string in second line");
    let bit_vector = string_to_bit_vector(bits);
    let aux_tables = build_aux_tables(&bit_vector);
    // Process each query and collect the results
    let results: Vec<u32> = (0..query_count)
        .map(|_| {
            split_contents
                .next()
                .expect("Too few queries in input file")
        })
        .map(|query_string| parse_and_run_query(&bit_vector, &aux_tables, query_string))
        .collect();
    let elapsed = now.elapsed();
    export_results(results, output_file_path);
    // Calculate space used by the bit vector (round up to next byte size)
    let bv_size = ((bit_vector.len() + 7) / 8) * size_of::<u32>();
    // Sum size of all auxiliary tables
    let aux_tables_size = (aux_tables.rank_0_table.len()
        + aux_tables.rank_1_table.len()
        + aux_tables.select_0_table.len()
        + aux_tables.select_1_table.len())
        * size_of::<usize>();
    let total_size = bv_size + aux_tables_size;
    println!(
        "RESULT algo=bv name=jonas_strittmatter time={:?} space={}",
        elapsed.as_millis(),
        total_size
    );
}

// Parses the query and executes the appropriate function depending on the query type.
fn parse_and_run_query(
    bit_vector: &Vec<u8>,
    aux_tables: &AuxiliaryTables,
    query_string: &str,
) -> u32 {
    let query_components: Vec<&str> = query_string.split(" ").collect();
    let query_type = query_components[0];
    let i = query_components[1].parse().unwrap();
    if query_type == "access" {
        return access(bit_vector, i);
    }
    let b = query_components[2].parse().unwrap();
    if query_type == "rank" {
        let rank_table = if b == 0 {
            &aux_tables.rank_0_table
        } else {
            &aux_tables.rank_1_table
        };
        return rank(bit_vector, rank_table, b, i);
    } else if query_type == "select" {
        let select_table = if b == 0 {
            &aux_tables.select_0_table
        } else {
            &aux_tables.select_1_table
        };
        return select(bit_vector, select_table, b, i);
    }
    panic!("Unexpected query {}", query_string);
}

// Returns the i-th bit in the bit vector.
fn access(bit_vector: &Vec<u8>, i: u32) -> u32 {
    // Determine the byte in which the i-th bit is located
    let block = bit_vector[(i / 8) as usize];
    // Determine the bit's offset within the byte
    let offset = 7 - (i % 8);
    let bit = (block >> offset) & 1;
    bit as u32
}

// Counts the number of occurrences of bit b (0 or 1) up to the i-th position.
// Uses a precomputed rank table for faster access.
fn rank(bit_vector: &Vec<u8>, rank_table: &Vec<usize>, b: u32, i: u32) -> u32 {
    let block_index = (i as usize) / BLOCK_SIZE;
    let mut rank = rank_table[block_index];
    let start = (block_index * BLOCK_SIZE) as u32;

    for j in start..i {
        if access(bit_vector, j) == b {
            rank += 1;
        }
    }
    rank as u32
}

// Finds the position of the i-th occurrence of bit b.
// Uses a precomputed select table for faster access.
// Returns u32::MAX if there is no such bit.
fn select(bit_vector: &Vec<u8>, select_table: &Vec<usize>, b: u32, i: u32) -> u32 {
    let mut count = 0;
    for &pos in select_table.iter() {
        if access(bit_vector, pos as u32) == b {
            count += 1;
            if count == i {
                return pos as u32;
            }
        }
    }
    // Return a sentinel value indicating not found
    u32::MAX
}

fn build_aux_tables(bit_vector: &Vec<u8>) -> AuxiliaryTables {
    // Stores all rank values at the beginning of a block
    let mut rank_0_table = vec![0; (bit_vector.len() * 8 + BLOCK_SIZE - 1) / BLOCK_SIZE];
    let mut rank_1_table = vec![0; (bit_vector.len() * 8 + BLOCK_SIZE - 1) / BLOCK_SIZE];
    // Stores the positions of all 0s / 1s in the bit vector
    let mut select_0_table = Vec::new();
    let mut select_1_table = Vec::new();
    let mut rank_0 = 0;
    let mut rank_1 = 0;

    for (i, &byte) in bit_vector.iter().enumerate() {
        for j in 0..8 {
            let jth_bit_in_byte = byte & (1 << (7 - j));
            if jth_bit_in_byte == 0 {
                rank_0 += 1;
                select_0_table.push(i * 8 + j);
            } else {
                rank_1 += 1;
                select_1_table.push(i * 8 + j);
            }
        }
        if (i * 8) % BLOCK_SIZE == 0 {
            rank_0_table[(i * 8) / BLOCK_SIZE] = rank_0;
            rank_1_table[(i * 8) / BLOCK_SIZE] = rank_1;
        }
    }
    AuxiliaryTables {
        rank_0_table,
        rank_1_table,
        select_0_table,
        select_1_table,
    }
}

// Converts a bit string to a vector of bytes.
fn string_to_bit_vector(bit_string: &str) -> Vec<u8> {
    let total_bytes = (bit_string.len() + 7) / 8;
    let mut bit_vector: Vec<u8> = Vec::with_capacity(total_bytes);
    let mut current_byte: u8 = 0;
    let mut bit_count = 0;

    for character in bit_string.chars() {
        // Left-shift current byte to make space for the new bit
        current_byte <<= 1;
        // Set the LSB of the current byte
        if character == '1' {
            current_byte |= 1;
        } else {
            current_byte |= 0;
        }
        bit_count += 1;
        // If 8 bits have been processed, push the current byte to the bit vector and reset
        if bit_count == 8 {
            bit_vector.push(current_byte);
            current_byte = 0;
            bit_count = 0;
        }
    }

    // Add the remaining bits if the bit count is not a multiple of 8
    if bit_count != 0 {
        current_byte <<= 8 - bit_count;
        bit_vector.push(current_byte);
    }
    bit_vector
}

// Writes the results to the output file (one line per query).
fn export_results(results: Vec<u32>, output_file_path: &str) {
    let results_string = results
        .iter()
        .map(|result| result.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(output_file_path, results_string).expect("Unable to write results to output file");
}
