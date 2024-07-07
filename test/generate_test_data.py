import random


def generate_bitvector(length):
    return "".join(random.choice("01") for _ in range(length))


def generate_queries(bitvector, num_queries=30):
    queries = []
    bit_length = len(bitvector)
    print("Generating access")
    for _ in range(10):
        index = random.randint(0, bit_length - 1)
        queries.append(f"access {index}")
    print("Generating rank")
    for _ in range(10):
        bit = random.choice("01")
        index = random.randint(0, bit_length - 1)
        queries.append(f"rank {bit} {index}")
    print("Generating select")
    count_0 = bitvector.count("0")
    count_1 = bitvector.count("1")
    for _ in range(10):
        bit = random.choice("01")
        rank = random.randint(1, count_0 if bit == "0" else count_1)
        queries.append(f"select {bit} {rank}")

    return queries


def access(bitvector, index):
    return int(bitvector[index])


def rank(bitvector, bit, index):
    bit = int(bit)
    return bitvector[:index].count(str(bit))


def select(bitvector, bit, rank):
    bit = str(bit)
    count = 0
    for i, b in enumerate(bitvector):
        if b == bit:
            count += 1
            if count == rank:
                return i
    assert False, "invalid select argument"


def main():
    random.seed(42)
    bitvector_length = 32
    bitvector = generate_bitvector(bitvector_length)

    queries = generate_queries(bitvector)
    num_queries = len(queries)

    with open("input_small_bv.txt", "w") as f:
        f.write(f"{num_queries}\n")
        f.write(bitvector + "\n")
        for query in queries:
            f.write(query + "\n")

    with open("expected_output_small_bv.txt", "w") as f:
        for query in queries:
            parts = query.split()
            if parts[0] == "access":
                result = access(bitvector, int(parts[1]))
            elif parts[0] == "rank":
                result = rank(bitvector, parts[1], int(parts[2]))
            elif parts[0] == "select":
                result = select(bitvector, parts[1], int(parts[2]))
            f.write(f"{result}\n")


if __name__ == "__main__":
    main()
