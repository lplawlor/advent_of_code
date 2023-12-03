schematic = open("input").read().splitlines()

# These are the dimensions of the engine schematic (input)
MAX_X = len(schematic[0]) - 1
MAX_Y = len(schematic) - 1


def is_symbol(char):
    """Return True if char is not a digit or a period."""
    return (not char.isdigit()) and char != "."


total = 0

for line_num, line in enumerate(schematic):
    num_start = 0

    # Scan along each line
    while num_start <= MAX_X:
        # Increment num_start until it is the index of the start of a number
        if not line[num_start].isdigit():
            num_start += 1
            continue

        # Find the end of the number
        # num_end is actually the index of the first non-digit character after the number
        num_end = num_start
        while num_end <= MAX_X and line[num_end].isdigit():
            num_end += 1

        is_part_number = False

        # For each x value between the start and end of the number, plus one column on either side
        # min() is used with MAX_X + 1 to catch the case where num_end is MAX_X + 1 (the number is at the edge)
        for x in range(num_start - 1, min(num_end + 1, MAX_X + 1)):
            # Get the character at the current x on the line above
            if line_num != 0:
                above_at_x = schematic[line_num - 1][x]
            else:
                # Fake a line above the schematic filled with periods
                above_at_x = "."

            # Get the character at the current x on the current line
            here_at_x = line[x]

            # Get the character at the current x on the line below
            if line_num != MAX_Y:
                below_at_x = schematic[line_num + 1][x]
            else:
                # Fake a line below the schematic filled with periods
                below_at_x = "."

            # If any of the characters are symbols (anything but numbers and '.')
            if is_symbol(above_at_x) or is_symbol(here_at_x) or is_symbol(below_at_x):
                # Set the is_part_number flag and break from the symbol-adjancency loop
                is_part_number = True
                break

        # If the number is adjacent to at least one symbol, add it to the sum
        if is_part_number:
            total += int(line[num_start:num_end])

        # Skip past this number and continue
        num_start = num_end

print(total)
