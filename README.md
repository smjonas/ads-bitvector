# ads-bitvector
An implementation of a bit vector with rank and select support, written in Rust.

## Installation
Requires Rust 1.75.0 or later (lower versions may work, but are untested).
To clone the repository: `git clone git@github.com:smjonas/ads-bitvector.git`.

## Usage
Run `cargo run --release -- <input_file> <output_file>` from the `ads_bitvector_block_based` directory of the repository,
e.g. `cargo run --release -- input/input.txt output/output.txt` to use the example input file from the exercise sheet.

## Running the Experiments
To run the experiments, navigate to the `eval` directory. Then run `python3 ./run_experiments <experiments>` where
`<experiments>` are one or more space-separated experiments. Valid experiment names are `gen_dataset`, `
benchmark_main`, `benchmark_block_based`,`plot_main`, `plot_block_based` and `block_combined`.
