def letter_to_score(char: str):
    if char in ("A", "X"):
        return 1
    elif char in ("B", "Y"):
        return 2
    else:
        return 3

lines = open("input").read().splitlines()

# Convert each letter to it's corresponding score
matches = [[letter_to_score(char) for char in line.split(" ")] for line in lines]

score = 0
for (elf, you) in matches:
    # You always score for the shape you select
    score += you

    if elf == you:
        score += 3
    
    # If your shape is 1 above elf's shape
    # (using mod 3 to wrap around so 1 is considered 1 above 3)
    elif (you - elf) % 3 == 1:
       score += 6 

print(score)