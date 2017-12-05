def count_jumps(memory):
    ip = 0
    jumps = 0
    while ip >= 0 and ip < len(memory):
        last_ip = ip
        ip += memory[ip]
        memory[last_ip] += 1
        jumps += 1
    return jumps

def count_wacky_jumps(memory):
    ip = 0
    jumps = 0
    while ip >= 0 and ip < len(memory):
        if len(memory) < 10: print(memory)
        last_ip = ip
        last_val = memory[ip]
        ip += last_val
        if last_val >= 3:
            memory[last_ip] = last_val - 1
            if len(memory) < 10:
                print('%s -> %s' % (last_ip, memory[last_ip]))
        else:
            memory[last_ip] = last_val + 1
            if len(memory) < 10:
                print('%s -> %s' % (last_ip, memory[last_ip]))
        jumps += 1
    return jumps

if __name__ == "__main__":
    print("Test: %s" % count_jumps([0, 3, 0, 1, -3]))
    puzzle = [int(line.strip()) for line in open("puzzle_data.txt")]
    print("Part 1: %s" % count_jumps(puzzle))

    print("Test Wacky: %s" % count_wacky_jumps([0, 3, 0, 1, -3]))
    puzzle = [int(line.strip()) for line in open("puzzle_data.txt")]
    print("Part 2: %s" % count_wacky_jumps(puzzle))
