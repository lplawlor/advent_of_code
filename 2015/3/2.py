radio = open("input").readline()

deliveries: set[tuple[int, int]] = set()

x, y = 0, 0
deliveries.add((x, y))

# Run through real-santas trip (skipping every even instruction)
for char in radio[::2]:
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

x, y = 0, 0
deliveries.add((x, y))

# Run through robo-santas trip (skipping every odd instruction)
for char in radio[1::2]:
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
