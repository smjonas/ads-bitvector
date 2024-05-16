import os
import random


def generate_bitvector(n, seed):
    random.seed(seed)
    return "".join(random.choice("01") for _ in range(n))


def index_bitvector(bitvector):
    indices = {"0": [], "1": []}
    for i, bit in enumerate(bitvector):
        indices[bit].append(i)
    return indices


def generate_query(indices, query_type, n):
    if query_type == "access":
        i = random.randint(0, n - 1)
        return f"access {i}"
    elif query_type == "rank":
        b = random.choice("01")
        i = random.randint(0, n - 1)
        return f"rank {b} {i}"
    elif query_type == "select":
        b = random.choice("01")
        count = len(indices[b])
        i = random.randint(1, count)
        return f"select {b} {i}"
    elif query_type == "mixed":
        query_type = random.choice(["access", "rank", "select"])
        return generate_query(indices, query_type, n)
    else:
        raise ValueError("Invalid query " + query_type)


def write_output_file(output_folder, filename, bitvector, queries):
    if not os.path.exists(output_folder):
        os.makedirs(output_folder)
    with open(os.path.join(output_folder, filename), "w") as f:
        f.write(f"{len(queries)}\n")
        f.write(f"{bitvector}\n")
        f.write("\n".join(queries))


def generate_dataset(n, max_k, step_size, query_type, output_folder, seed):
    bitvector = generate_bitvector(n, seed)
    indices = index_bitvector(bitvector)

    for k in range(step_size, max_k + 1, step_size):
        queries = []
        while len(queries) < k:
            query = generate_query(indices, query_type, n)
            queries.append(query)

        filename = f"bitvector_{query_type}_n{n}_seed{seed}_queries{k}.txt"
        write_output_file(output_folder, filename, bitvector, queries)


if __name__ == "__main__":
    import sys

    args = sys.argv[1:]
    if len(args) != 6:
        print("Usage: python script.py <n> <max_k> <step_size> <query_type> <output_folder> <seed>")
    else:
        n = int(args[0])
        max_k = int(args[1])
        step_size = int(args[2])
        query_type = args[3]
        output_folder = args[4]
        seed = int(args[5])
        generate_dataset(n, max_k, step_size, query_type, output_folder, seed)
