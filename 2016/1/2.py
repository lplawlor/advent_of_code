steps = open("./input").readline().split(", ")

# NESW -> 0123
facing = 0
x = 0
y = 0

visited = []

for step in steps:
    right = step[0] == "R"
    distance = int(step[1:])

    # Use mod 4 to keep facing direction within 0-3
    facing = facing + 1 if right else facing - 1
    facing %= 4

    # Go step by step along the distance in the facing direction
    for _ in range(1, distance + 1):
        if facing == 0:
            y += 1
        elif facing == 1:
            x += 1
        elif facing == 2:
            y -= 1
        else:
            x -= 1

        # If this location has been visited before
        if (x, y) in visited:
            # Done. Print taxi-cab distance and exit
            print(abs(x) + abs(y))
            exit()

        # Add the new location to the list
        visited.append((x, y))
