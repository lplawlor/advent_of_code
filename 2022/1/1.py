lines = open("input").read().splitlines()

max_calories = 0
calories = 0
for line in lines:
    if line:
        calories += int(line)
    else:
        max_calories = max(max_calories, calories)
        calories = 0
        
print(max_calories)