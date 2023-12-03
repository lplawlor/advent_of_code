ids = open("input").read().splitlines()


def errors(id1: str, id2: str):
    """Get the number of characters that differ between two equal-length strings"""
    total = 0

    for c1, c2 in zip(id1, id2):
        if c1 != c2:
            total += 1

    return total


def without_errors(id1: str, id2: str):
    """Remove the characters that differ between two equal-length strings and return the result"""
    clean = ""

    for c1, c2 in zip(id1, id2):
        if c1 == c2:
            clean += c1

    return clean


for i in range(len(ids)):
    for j in range(i, len(ids)):
        id1 = ids[i]
        id2 = ids[j]
        if errors(id1, id2) == 1:
            print(without_errors(id1, id2))
            exit()
