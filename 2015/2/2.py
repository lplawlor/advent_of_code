lines = open("./input").read().splitlines()

# Split each line by the "x" and convert the dimensions to ints
dims = [[int(dim) for dim in line.split("x")] for line in lines]

ribbon = 0

for w, l, h in dims:
    p1 = 2 * w + 2 * l
    p2 = 2 * w + 2 * h
    p3 = 2 * l + 2 * h
    v = w * l * h

    ribbon += min(p1, p2, p3) + v

print(ribbon)
