total = 0

with open("./input", "r") as f:
    for line in f.readlines():
        # Search forward direction
        for char in line:
            if not char.isdigit():
                continue
            total += 10 * int(char)
            break

        # Search reverse direction
        for char in line[::-1]:
            if not char.isdigit():
                continue
            total += int(char)
            break

print(total)
