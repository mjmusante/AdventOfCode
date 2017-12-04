def count_valid(lines, anagram=False):
    total = 0
    for l in lines:
        valid = True
        x = dict()
        for w in l.split():
            if anagram:
                w = "".join(sorted(w))
            if w in x:
                valid = False
                break
            else:
                x[w] = 1
        if valid:
            total += 1
    return total


if __name__ == "__main__":
    puzzle = [line.strip() for line in open("puzzle_data.txt")]
    print("Part 1: %s" % count_valid(puzzle))
    print("Part 2: %s" % count_valid(puzzle, True))