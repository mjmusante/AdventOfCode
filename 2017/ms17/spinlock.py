def do_spin(lim, stop):
    mem = [0]
    for i in range(1, stop + 1):
        x = (lim + 1) % i
        mem = [i] + mem[x:] + mem[:x]
    n = mem.index(0)
    return mem[n:] + mem[:n]


def do_search(lim, stop):
    loc = 0
    ans = 0
    for i in range(1, stop):
        loc = (loc + (lim + 1)) % (i + 1)
        if loc == 0:
            ans = i + 1
    return ans


if __name__ == "__main__":

    rslt = do_spin(370, 2017)
    print("Part 1: %s" % rslt[rslt.index(2017) + 1])
    print("Part 2: %s" % do_search(370, 50000000))
