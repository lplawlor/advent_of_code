schematic = open("input").read().splitlines()

# These are the dimensions of the engine schematic (input)
MAX_X = len(schematic[0]) - 1
MAX_Y = len(schematic) - 1

total = 0

# This dict will have the coordinates of the potential gears as the keys
# and the tuple (gear_ratio, adjacent_number_count) as the values
# In the end, a potential gear is only really a gear if adjacent_number_count = 2
potential_gears: dict[tuple[int, int] : tuple[int, int]] = dict()


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

            # If a potential gear was found on the row above
            if above_at_x == "*":
                # Get the potential_gears entry, defaulting to gear_ratio = 1, adjacent_number_count = 0 if not found
                gear_ratio, adjacent_number_count = potential_gears.get(
                    (line_num - 1, x), (1, 0)
                )

                # Update the potential_gear entry by multiplying the gear ratio by the number and incrementing the count
                potential_gears[(line_num - 1, x)] = (
                    gear_ratio * int(line[num_start:num_end]),
                    adjacent_number_count + 1,
                )

            # If a potential gear was found on the current row
            if here_at_x == "*":
                # Get the potential_gears entry, defaulting to gear_ratio = 1, adjacent_number_count = 0 if not found
                gear_ratio, adjacent_number_count = potential_gears.get(
                    (line_num, x), (1, 0)
                )

                # Update the potential_gear entry by multiplying the gear ratio by the number and incrementing the count
                potential_gears[(line_num, x)] = (
                    gear_ratio * int(line[num_start:num_end]),
                    adjacent_number_count + 1,
                )

            # If a potential gear was found on the row below
            if below_at_x == "*":
                # Get the potential_gears entry, defaulting to gear_ratio = 1, adjacent_number_count = 0 if not found
                gear_ratio, adjacent_number_count = potential_gears.get(
                    (line_num + 1, x), (1, 0)
                )

                # Update the potential_gear entry by multiplying the gear ratio by the number and incrementing the count
                potential_gears[(line_num + 1, x)] = (
                    gear_ratio * int(line[num_start:num_end]),
                    adjacent_number_count + 1,
                )

        # Skip past this number and continue
        num_start = num_end

for gear_ratio, adjacent_number_count in potential_gears.values():
    # Only sum up gear_ratios for real gears
    # I.e. "*" adjacent to exactly 2 numbers
    if adjacent_number_count == 2:
        total += gear_ratio

print(total)
