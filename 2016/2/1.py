instructions = open("input").read().splitlines()

buttons = [
    ["1", "2", "3"],
    ["4", "5", "6"],
    ["7", "8", "9"],
]

# Use x,y coords where the 5 button is (1,1)
x, y = 1, 1

code = ""
for line in instructions:
    for char in line:
        if char == "U" and y != 0:
            y -= 1
        elif char == "D" and y != 2:
            y += 1
        elif char == "L" and x != 0:
            x -= 1
        elif char == "R" and x != 2:
            x += 1

    code += buttons[y][x]

print(code)
