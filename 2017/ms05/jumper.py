def count_jumps(memory):
    ip = 0
    jumps = 0
    while ip >= 0 and ip < len(memory):
        memory[ip] += 1
        ip += memory[ip] - 1
        jumps += 1
    return jumps


def count_wacky_jumps(memory):
    ip = 0
    jumps = 0
    while ip >= 0 and ip < len(memory):
        if memory[ip] >= 3:
            memory[ip] -= 1
            ip += memory[ip] + 1
        else:
            memory[ip] += 1
            ip += memory[ip] - 1
        jumps += 1
    return jumps


if __name__ == "__main__":
    # print("Test: %s" % count_jumps([0, 3, 0, 1, -3]))
    puzzle = [int(line.strip()) for line in open("puzzle_data.txt")]
    print("Part 1: %s" % count_jumps(puzzle))

    # print("Test Wacky: %s" % count_wacky_jumps([0, 3, 0, 1, -3]))
    puzzle = [int(line.strip()) for line in open("puzzle_data.txt")]
    print("Part 2: %s" % count_wacky_jumps(puzzle))
