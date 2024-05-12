import run

n = 1_000_000
step_size = 25_000

run.add(
    "gen_dataset",
    f"python3 generate_dataset.py {n} {step_size} [[query_type]] ./dataset [[seed]]",
    {"query_type": ["access", "rank", "select", "mixed"], "seed": ["42", "43", "44", "45"]},
    creates_file=f"./dataset/bitvector_[[query_type]]_n{n}_seed[[seed]]_queries{step_size}.txt",
)

run.add(
    "benchmark",
    f"python3 benchmark.py ./dataset/bitvector_[[query_type]]_n{n}_seed[[seed]]_queries[[k]].txt",
    {
        "query_type": ["access", "rank", "select", "mixed"],
        "seed": ["42", "43", "44", "45"],
        "k": list(range(step_size, n + 1, step_size)),
    },
    header_command="python3 benchmark.py --header",
    stdout_file="./output/benchmark_results.csv",
)

run.add(
    "plot",
    "python3 plot_query_type_vs_time.py ./output/benchmark_results.csv ./output/query_type_vs_time.png",
    {},
)

run.run()
