def do_spin(lim, stop):
    mem = [0]
    for i in range(1, stop + 1):
        x = (lim + 1) % len(mem)
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

rslt = do_spin(3, 2017)
print("Test: %s" % rslt[rslt.index(2017)+1])

rslt = do_spin(370, 2017)
print("Part 1: %s" % rslt[rslt.index(2017)+1])

print("Part 2: %s" % do_search(370, 50000000))
