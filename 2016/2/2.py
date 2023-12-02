instructions = open("input").read().splitlines()

buttons = [
    [" ", " ", "1", " ", " "],
    [" ", "2", "3", "4", " "],
    ["5", "6", "7", "8", "9"],
    [" ", "A", "B", "C", " "],
    [" ", " ", "D", " ", " "],
]

# Use x,y coords where the 5 button is (0,2)
x, y = 0, 2

code = ""
for line in instructions:
    for char in line:
        if char == "U" and y != 0 and buttons[y - 1][x] != " ":
            y -= 1
        elif char == "D" and y != 4 and buttons[y + 1][x] != " ":
            y += 1
        elif char == "L" and x != 0 and buttons[y][x - 1] != " ":
            x -= 1
        elif char == "R" and x != 4 and buttons[y][x + 1] != " ":
            x += 1

    code += buttons[y][x]

print(code)
