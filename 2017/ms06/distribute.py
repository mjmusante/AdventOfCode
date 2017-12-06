def distribute(memory):
    seen = dict()
    size = len(memory)
    iterations = 0

    while tuple(memory) not in seen:
        seen[tuple(memory)] = iterations
        iterations += 1
        maxval = max(memory)
        maxidx = memory.index(maxval)
        memory[maxidx] = 0
        while maxval > 0:
            maxidx = (maxidx + 1) % size
            memory[maxidx] += 1
            maxval -= 1

    dist = abs(seen[tuple(memory)] - iterations)
    return (iterations, dist)


print("Test data: cycles=%s, distance=%s" % distribute([0, 2, 7, 0]))
puzzle_data = [line.strip() for line in open("puzzle_data.txt")]
puzzle = [int(val) for val in puzzle_data[0].split()]
print("Part 1 & 2: cycles=%s, distance=%s" % distribute(puzzle))