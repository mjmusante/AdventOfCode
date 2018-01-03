def pattern(tops):
    pos = 0
    f = range(1, tops + 1)
    while len(f) > 1:
        # print(f)
        steal = (int(len(f) / 2)) % len(f)
        # print("%s stealing %s" % (f[0], f[steal]))
        f.remove(f[steal])
        f = f[1:] + [f[0]]
    return f[0]


def josephus_for(value):
    # twice the difference between the value and highest power of 2
    # plus 1
    n = 1
    while n < value:
        n *= 2
    return 2 * (value - n / 2) + 1


def circular_josephus(value):
    # using the pattern() function above, it was clear to see that
    # it was based on powers of 3. For each power of 3 (1, 3, 9, 27, ...)
    # the 'winner' is the player in the last position. e.g. for 27 Elves,
    # the Elf in position 27 wins.
    #
    # The second part of the pattern showed that additional Elves move
    # the winner forward by 1. E.g., for 28 Elves, the winner is 1; for 29
    # the winner is 2, and so on.
    #
    # This pattern holds true until we double the original power of 3, at
    # which point it skips one additional Elf. E.g. for power of 3 'n',
    # if there are n + m Elves and m is less than n, then the winner is m.
    # But for n + m elves where m > n, the winner is n + 2m.
    if value == 1:
        return 1

    n = 1
    while n < value:
        n *= 3
    n /= 3
    if value <= 2 * n:
        return value - n
    # return (value - n) + (value - 2 * n)
    return 2 * value - 3 * n


if __name__ == "__main__":
    print("Part 1: %s" % josephus_for(3014603))
    print("Part 2: %s" % circular_josephus(3014603))
