# Remove the initial "Game <n>: " prefix from each line
# We can determine game number by index + 1
lines = [line.split(": ", 1)[1] for line in open("input").read().splitlines()]

total = 0

for game, line in enumerate(lines, start=1):
    subsets = line.split("; ")

    # These are the maximum counts found for this game
    max_r = 0
    max_g = 0
    max_b = 0

    for subset in subsets:
        # Each element of data looks like "10 red", "1 green", etc.
        data = subset.split(", ")

        for datum in data:
            count, color = datum.split(" ")
            count = int(count)

            if color == "red":
                max_r = max(max_r, count)
            elif color == "green":
                max_g = max(max_g, count)
            else:
                max_b = max(max_b, count)

    total += max_r * max_g * max_b

print(total)
