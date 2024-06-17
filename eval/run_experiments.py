import run

n = 1_000_000
max_k = 200_000
step_size = 10_000
seed = 42

run.add(
    "gen_dataset",
    f"python3 generate_dataset.py {n} {max_k} {step_size} [[query_type]] ./dataset {seed}",
    {"query_type": ["access", "rank", "select", "mixed"]},
    creates_file=f"./dataset/bitvector_[[query_type]]_n{n}_seed{seed}_queries{step_size}.txt",
)

implementations = ["main", "block_based"]
project_names = ["ads_bitvector", "ads_bitvector_block_based"]
for implementation, project_name in zip(implementations, project_names):
    run.add(
        f"benchmark_{implementation}",
        f"python3 benchmark.py {project_name} ./dataset/bitvector_[[query_type]]_n{n}_seed{seed}_queries[[k]].txt",
        {
            "query_type": ["access", "rank", "select", "mixed"],
            "k": list(range(step_size, max_k + 1, step_size)),
        },
        header_command="python3 benchmark.py --header",
        stdout_file=f"./output/benchmark_results_{implementation}.csv",
    )

    run.add(
        f"plot_{implementation}",
        f"python3 plot_query_type_vs_time.py ./output/benchmark_results_{implementation}.csv ./output/query_type_vs_time_{implementation}.png",
        {},
    )

plot_combined_args = " ".join(
    [
        f"./output/benchmark_results_{implementation}.csv {implementation}"
        for implementation in implementations
    ]
)
run.add(
    "plot_combined",
    f"python3 plot_query_type_vs_time_combined.py ./output/query_type_vs_time_combined.png {plot_combined_args}",
    {},
)

run.run()
