import run

run.add(
    "gen_dataset",
    "python3 generate_dataset.py 1_000_000 25_000 [[query_type]] ./input [[seed]]",
    {"query_type": ["access", "rank", "select", "mixed"], "seed": ["42", "43", "44", "45"]},
    creates_file="./input/bitvector_[[query_type]]_n1000000_seed[[seed]]_queries25000.txt",
)
run.run()
