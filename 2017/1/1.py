digits = open("input").readline().strip()

total = 0

size = len(digits)

# For every digit except the last
for i in range(size):
    if digits[i] == digits[(i+1) % size]:
        total += int(digits[i])

print(total)
