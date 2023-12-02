steps = open("./input").readline().split(", ")

# NESW -> 0123
facing = 0
x = 0
y = 0

for step in steps:
    right = step[0] == "R"
    distance = int(step[1:])

    # Use mod 4 to keep facing direction within 0-3
    facing = facing + 1 if right else facing - 1
    facing %= 4

    # Jump along the facing direction by the distance
    if facing == 0:
        y += distance
    elif facing == 1:
        x += distance
    elif facing == 2:
        y -= distance
    else:
        x -= distance

# Print taxi-cab distance
print(abs(x) + abs(y))
