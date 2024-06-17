import sys

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

colors = ["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728"]  # Blue, Orange, Green, Red


def plot_results(results_files, output_path):
    plt.figure(figsize=(10, 6))

    for results_file, kind in results_files:
        df = pd.read_csv(results_file)
        query_types = df["query_type"].unique()

        assert len(query_types) == len(colors)
        for query_type, color in zip(query_types, colors):
            subset = df[df["query_type"] == query_type]
            linestyle = "-" if "main" in kind else "--"
            plt.plot(
                subset["k"],
                subset["time"],
                label=f"{query_type} ({kind})",
                color=color,
                linestyle=linestyle,
            )

    plt.xlabel("Number of queries")
    plt.ylabel("Time (ms)")
    plt.legend()
    plt.grid(True)
    plt.savefig(output_path)
    plt.show()


if __name__ == "__main__":
    if len(sys.argv) < 4 or len(sys.argv) % 2 != 0:
        print(
            "Usage: python plot_query_type_vs_time_combined.py <output_path> <results_csv_1> <kind_1> [<results_csv_2> <kind_2> ...]"
        )
        sys.exit(1)

    output_path = sys.argv[1]
    results_files = [(sys.argv[i], sys.argv[i + 1]) for i in range(2, len(sys.argv), 2)]
    plot_results(results_files, output_path)
