import p274_h_index


def main():
    solution = p274_h_index.Solution

    citations: list[int] = [0, 0]
    value = solution.hIndex(solution, citations)
    print(value)


if __name__ == "__main__":
    main()
