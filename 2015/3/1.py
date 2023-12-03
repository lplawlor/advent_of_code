radio = open("input").readline()

deliveries: set[tuple[int, int]] = set()

x, y = 0, 0
deliveries.add((x, y))

for char in radio:
    match char:
        case ">":
            x += 1
        case "<":
            x -= 1
        case "^":
            y += 1
        case _:
            y -= 1

    deliveries.add((x, y))

print(len(deliveries))
