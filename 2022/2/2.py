"""
|___|_A_|_B_|_C_|
| X | C | A | B |
| Y | A | B | C |
| Z | B | C | A |
"""

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
for (elf, result) in matches:
    if result == 1:
        score += (elf + 1) % 3 + 1

    elif result == 2:
        score += 3
        score += elf
    
    else:
        score += 6
        score += elf % 3 + 1
    
print(score)