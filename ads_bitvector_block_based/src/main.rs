use std::cmp::min;
use std::fs;
use std::mem::size_of;
use std::time::Instant;

const BLOCK_SIZE: usize = 512;

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
    let (rank0_table, rank1_table) = build_rank_tables(&bit_vector);
    // Process each query and collect the results
    let results: Vec<u32> = (0..query_count)
        .map(|_| {
            split_contents
                .next()
                .expect("Too few queries in input file")
        })
        .map(|query_string| {
            parse_and_run_query(&bit_vector, &rank0_table, &rank1_table, query_string)
        })
        .collect();
    let elapsed = now.elapsed();
    export_results(results, output_file_path);
    // Calculate space used by the bit vector (round up to next byte size)
    let bv_size = ((bit_vector.len() + 7) / 8) * size_of::<u32>();
    // Sum size of all auxiliary tables
    let aux_tables_size = rank0_table.len() * size_of::<usize>();
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
    rank0_table: &Vec<u32>,
    rank1_table: &Vec<u32>,
    query_string: &str,
) -> u32 {
    println!("query: {}", query_string);
    let query_components: Vec<&str> = query_string.split(" ").collect();
    let query_type = query_components[0];
    let i = query_components[1].parse().unwrap();
    if query_type == "access" {
        return access(bit_vector, i);
    }
    let b = i;
    let i = query_components[2].parse().unwrap();
    let rank_table = if b == 0 { &rank0_table } else { &rank1_table };
    if query_type == "rank" {
        return rank(bit_vector, rank_table, b, i);
    } else if query_type == "select" {
        return select(bit_vector, rank_table, b, i).expect("invalid argument 'i' in select query");
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
fn rank(bit_vector: &Vec<u8>, rank_table: &Vec<u32>, b: u32, i: u32) -> u32 {
    let block_index = (i as usize) / BLOCK_SIZE;
    let mut rank = rank_table[block_index];
    let start = (block_index * BLOCK_SIZE * 8) as u32;

    for j in start..i {
        if access(bit_vector, j) == b {
            println!("rank: {}", rank);
            rank += 1;
        }
    }
    rank as u32
}

// Finds the position of the i-th occurrence of bit b.
// Uses the precomputed rank tables for faster access.
// Returns None if there is no such bit.
fn select(bit_vector: &Vec<u8>, rank_table: &Vec<u32>, b: u32, i: u32) -> Option<u32> {
    // The block to search in; if the rank at a block boundary is higher than i, then we know it we went too far
    println!("{:?}", rank_table);
    let block = find_predecessor_index(rank_table, i).expect("invalid argument 'i' in select query");
    println!("block: {}", block);
    // The initial count is given by the rank at the start of the block
    let mut count = rank_table[block as usize];
    for bit_offset in 0..min(BLOCK_SIZE, bit_vector.len() * 8) as u32 {
        let pos = 8 * (block * BLOCK_SIZE) as u32 + bit_offset;
        println!("pos: {}, offset: {}, len: {}, b: {}", pos, bit_offset, bit_vector.len() * 8, b);
        if access(bit_vector, pos) == b {
            count += 1;
            println!("count: {}", count);
            if count == i {
                println!("found! {}", pos);
                return Some(pos);
            }
        }
    }
    // Not found (should not occur for valid values of i)
    None
}

// Returns the index of the largest value <= x, or None if not found.
// Requires O(n) time for simplicity, which could be improved.
fn find_predecessor_index(values: &Vec<u32>, x: u32) -> Option<usize> {
    let mut predecessor = None;
    for (i, &value) in values.iter().enumerate() {
        if value <= x {
            predecessor = Some(i)
        } else {
            break;
        }
    }
    predecessor
}

// Constructs auxiliary rank tables to speed-up queries.
fn build_rank_tables(bit_vector: &Vec<u8>) -> (Vec<u32>, Vec<u32>) {
    // Stores all rank_0 / rank_1 values at the beginning of a block
    let mut rank0_table = vec![0; (bit_vector.len() * 8 + BLOCK_SIZE - 1) / BLOCK_SIZE];
    let mut rank1_table = vec![0; (bit_vector.len() * 8 + BLOCK_SIZE - 1) / BLOCK_SIZE];
    // Stores the number of all 0s / 1s in the bit vector
    let mut rank0 = 0;
    let mut rank1 = 0;

    for block in 1..rank0_table.len() {
        for i in 0..BLOCK_SIZE * 8 {
            let bit = access(bit_vector, (block * BLOCK_SIZE * 8 + i) as u32);
            if bit == 0 {
                rank0 += 1;
            } else {
                rank1 += 1;
            }
        }
        rank0_table[block] = rank0;
        rank1_table[block] = rank1;
    }
    println!("rank0 table: {:?}", rank0_table);
    println!("rank1 table: {:?}", rank1_table);
    (rank0_table, rank1_table)
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
