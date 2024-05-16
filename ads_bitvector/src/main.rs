use std::fs;
use std::mem::size_of;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    let input_file_path = &args[1];
    let output_file_path = &args[2];
    let contents = std::fs::read_to_string(input_file_path).expect("Failed to read input file");
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
    let results: Vec<u32> = (0..query_count)
        .map(|_| {
            split_contents
                .next()
                .expect("Too few queries in input file")
        })
        .map(|query_string| parse_and_run_query(&bit_vector, query_string))
        .collect();
    let elapsed = now.elapsed();
    export_results(results, output_file_path);
    let size = ((bit_vector.len() + 7) / 8) * size_of::<u32>();
    println!(
        "RESULT algo=bv name=jonas_strittmatter time={:?} space={}",
        elapsed.as_millis(),
        size
    );
}

fn parse_and_run_query(bit_vector: &Vec<u8>, query_string: &str) -> u32 {
    let query_components: Vec<&str> = query_string.split(" ").collect();
    let query_type = query_components[0];
    let first_arg = query_components[1].parse().unwrap();
    let result = match query_type {
        "access" => access(bit_vector, first_arg),
        "rank" => rank(bit_vector, first_arg, query_components[2].parse().unwrap()),
        "select" => select(bit_vector, first_arg, query_components[2].parse().unwrap()),
        _ => panic!("Unexpected query {}", query_string),
    };
    result
}

fn access(bit_vector: &Vec<u8>, i: u32) -> u32 {
    let block = bit_vector[(i / 8) as usize];
    let offset = 7 - (i % 8);
    let byte = (block >> offset) & 1;
    byte as u32
}

fn rank(bit_vector: &Vec<u8>, b: u32, i: u32) -> u32 {
    (0..i)
        .map(|j| access(&bit_vector, j))
        .filter(|&val| val == b)
        .count() as u32
}

fn select(bit_vector: &Vec<u8>, b: u32, i: u32) -> u32 {
    let mut count = 0;
    let mut pos = 0;
    while count < i {
        if access(bit_vector, pos) == b {
            count += 1;
        }
        pos += 1
    }
    pos - 1
}

fn export_results(results: Vec<u32>, output_file_path: &str) {
    let results_string = results
        .iter()
        .map(|result| result.to_string())
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(output_file_path, results_string).expect("Unable to write results to output file");
}

fn string_to_bit_vector(bit_string: &str) -> Vec<u8> {
    let total_bytes = (bit_string.len() + 7) / 8;
    let mut bit_vector: Vec<u8> = Vec::with_capacity(total_bytes);
    let mut current_byte: u8 = 0;
    let mut bit_count = 0;

    for character in bit_string.chars() {
        current_byte <<= 1;
        if character == '1' {
            current_byte |= 1;
        } else {
            current_byte |= 0;
        }
        bit_count += 1;
        if bit_count == 8 {
            bit_vector.push(current_byte);
            current_byte = 0;
            bit_count = 0;
        }
    }

    if bit_count != 0 {
        current_byte <<= 8 - bit_count;
        bit_vector.push(current_byte);
    }
    bit_vector
}
