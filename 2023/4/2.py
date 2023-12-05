cards = [[len(set([num for num in line.split(":")[1].split(" | ")[0].split(" ")if num != ""]).intersection(set([num for num in line.split(":")[1].split(" | ")[1].split(" ")if num != ""]))), 1] for line in open("input").read().splitlines()]

for i, (matches, count) in enumerate(cards):
    for j in range(i + 1, min(i + 1 + matches, len(cards))):
        cards[j][1] += count

print(sum([card[1] for card in cards]))