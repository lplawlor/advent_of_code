nums = [[int(n) for n in l.split("\t")] for l in open("input").read().splitlines()]

total = 0

def only_division(line: list[int]):
    """Find the result of the only possible clean division of two numbers in the line"""
    for i in range(len(line)):
        for j in range(i + 1, len(line)):
            num1 = max(line[i], line[j])
            num2 = min(line[i], line[j])

            if num1 % num2 == 0:
                return num1 // num2

for line in nums:
    total += only_division(line)

print(total)