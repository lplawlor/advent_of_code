lines = open("./input").read().splitlines()

# Split each line by the "x" and convert the dimensions to ints
dims = [[int(dim) for dim in line.split("x")] for line in lines]

paper = 0

for w, l, h in dims:
    s1 = w * l
    s2 = w * h
    s3 = l * h

    paper += 2 * s1 + 2 * s2 + 2 * s3 + min(s1, s2, s3)

print(paper)
