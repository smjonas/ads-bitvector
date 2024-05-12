import sys

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


def plot_results(results_csv, output_path):
    df = pd.read_csv(results_csv)
    plt.figure(figsize=(10, 6))
    query_types = df["query_type"].unique()
    colors = plt.cm.viridis(np.linspace(0, 1, len(query_types)))

    for query_type, color in zip(query_types, colors):
        subset = df[df["query_type"] == query_type]
        plt.plot(subset["k"], subset["time"], label=query_type, color=color)

    plt.xlabel("Number of queries")
    plt.ylabel("Time (ms)")
    plt.title("Runtime by query type (n = 1.000.000)")
    plt.legend()
    plt.show()
    plt.savefig(output_path)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: python plot_benchmarks.py <results_csv> <output_path>")
        sys.exit(1)

    results_csv = sys.argv[1]
    output_path = sys.argv[2]
    plot_results(results_csv, output_path)
