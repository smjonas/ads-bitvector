import sys

import matplotlib.pyplot as plt
import pandas as pd

colors = ["#1f77b4", "#ff7f0e", "#2ca02c", "#d62728"]  # Blue, Orange, Green, Red


def plot_results(results_csv, output_path):
    df = pd.read_csv(results_csv)
    plt.figure(figsize=(10, 6))
    query_types = df["query_type"].unique()

    assert len(query_types) == len(colors)
    for query_type, color in zip(query_types, colors):
        subset = df[df["query_type"] == query_type]
        plt.plot(subset["k"], subset["time"], label=query_type, color=color)

    plt.xlabel("Number of queries")
    plt.ylabel("Time (ms)")
    plt.legend()
    plt.savefig(output_path)
    plt.show()


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python plot_benchmarks.py <results_csv> <output_path>")
        sys.exit(1)

    results_csv = sys.argv[1]
    output_path = sys.argv[2]
    plot_results(results_csv, output_path)
