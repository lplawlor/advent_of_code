NUMS = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
REVERSED_NUMS = [num[::-1] for num in NUMS]


def scan_for_num(line: str, reverse: bool = False):
    """Get the first number in the line, or the last if reverse=True"""
    # Reverse the line and all the number names if reverse=True
    line = line[::-1] if reverse else line
    num_list = REVERSED_NUMS if reverse else NUMS

    # Store the best index for a spelled-out number
    best_index = len(line)
    # (and it's value)
    best_num = None

    # For each spelled-out number
    for n, s in enumerate(num_list):
        try:
            # Find the index of the substring
            index = line.index(s)

            # If it's the best yet, update the record
            if index < best_index:
                best_index = index
                best_num = n + 1

        except ValueError:
            # ValueError is raised if no substring found
            pass

    # Now run through all the characters BEFORE the best
    # spelled-out number to search for an actual number
    # I feel like it's blatantly better to do this step first,
    # but I'm not too concerned with efficiency here
    for char in line[:best_index]:
        if not char.isdigit():
            continue
        best_num = int(char)
        break

    return best_num


total = 0

with open("./input", "r") as f:
    for line in f.readlines():
        total += 10 * scan_for_num(line)
        total += scan_for_num(line, reverse=True)

print(total)
