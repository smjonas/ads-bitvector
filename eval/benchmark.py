import os
import re
import subprocess
import sys


def parse_result(output):
    parts = output.strip().split()
    time = next(
        (int(part.split("=")[1]) for part in parts if part.startswith("time=")), None
    )
    space = next(
        (int(part.split("=")[1]) for part in parts if part.startswith("space=")), None
    )
    return time, space


def benchmark_rust_program(project_name, input_file, output_file):
    command = f'cargo run --release --manifest-path="../{project_name}/Cargo.toml" -- {input_file} {output_file}'
    result = subprocess.run(command, shell=True, capture_output=True, text=True)
    if result.returncode != 0:
        print(result.stderr)
        sys.exit(1)
    return parse_result(result.stdout)


def extract_k(filename):
    match = re.search(r"queries(\d+)\.txt", filename)
    if match:
        return int(match.group(1))
    else:
        return None


def main(project_name, input_file) -> str:
    query_type = input_file.split("_")[1]
    k = extract_k(input_file)
    output_file = "temp_output.txt"
    time, space = benchmark_rust_program(project_name, input_file, output_file)
    file_name = os.path.basename(input_file)
    return f"{query_type},{file_name},{k},{time},{space}"


def print_header():
    print("query_type,input_file,k,time,space")


if __name__ == "__main__":
    args = [arg for arg in sys.argv[1:] if not arg.startswith("--")]
    flags = [arg for arg in sys.argv[1:] if arg.startswith("--")]
    if "--header" in flags:
        print_header()
        sys.exit(0)

    if len(sys.argv) != 3:
        print("Usage: python benchmark.py <project_name> <input_file> [--header]")
        sys.exit(1)

    project_name = sys.argv[1]
    input_file = sys.argv[2]
    result = main(project_name, input_file)
    print(result)
