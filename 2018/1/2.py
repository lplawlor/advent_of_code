reached = set()

freq = 0

with open("input", "r") as f:
    nums = [int(n) for n in f.read().splitlines()]
    while True:
        for num in nums:
            reached.add(freq)
            freq += num
            if freq in reached:
                print(freq)
                exit()
