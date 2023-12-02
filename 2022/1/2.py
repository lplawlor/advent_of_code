lines = open("input").read().splitlines()

elves = []
calories = 0
for line in lines:
    if line:
        calories += int(line)
    else:
        elves.append(calories)
        calories = 0
        
elves.sort()
print(sum(elves[-3:]))