line = open("./input").readline()
floor = 0
for position, char in enumerate(line, 1):
    floor = floor + 1 if char == "(" else floor - 1
    if floor < 0:
        print(position)
        exit()
