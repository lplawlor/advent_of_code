ids = open("input").read().splitlines()

twice = 0
thrice = 0
for id in ids:
    # Flags for if id contains a character repeated exactly twice and thrice respectively
    two = False
    three = False

    # Use set to run through each unique char in the id
    for char in set(id):
        count = id.count(char)

        if count == 3:
            three = True
            # If a char with count 2 has already been found
            if two:
                # We're done with this id
                break
        if count == 2:
            two = True
            # If a char with count 3 has already been found
            if three:
                # We're done with this id
                break

    # Increase the sum(s) if the flag(s) were set
    if two:
        twice += 1
    if three:
        thrice += 1

print(twice * thrice)
