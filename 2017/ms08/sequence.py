import operator
import re

DECODE = re.compile(
    r'^([a-z]*) (inc|dec) (-?\d*) if ([a-z]*) ([^ ]*) (-?[\d]*)$')


def largest(ary):
    reg = dict()
    highest = None
    icount = 0
    rcount = 0
    for i in ary:
        icount += 1
        m = DECODE.match(i)
        if not m:
            print("*parse fail* '%s'" % i)
            return (None, None)
        prime = m.group(1)          # b
        inst = m.group(2)           # inc
        amount = int(m.group(3))    # 5 (if)
        compare = m.group(4)        # a
        how = m.group(5)            # >
        val = int(m.group(6))       # 1
        if compare not in reg:
            reg[compare] = 0
            rcount += 1
        if prime not in reg:
            reg[prime] = 0
            rcount += 1
        if how == "<":
            do_it = reg[compare] < val
        elif how == "<=":
            do_it = reg[compare] <= val
        elif how == "==":
            do_it = reg[compare] == val
        elif how == "!=":
            do_it = reg[compare] != val
        elif how == ">=":
            do_it = reg[compare] >= val
        elif how == ">":
            do_it = reg[compare] > val
        else:
            print("*comparison fail* '%s'" % how)
            return (None, None)
        if do_it:
            if inst == "inc":
                reg[prime] += amount
            elif inst == "dec":
                reg[prime] -= amount
            else:
                print("*instruction fail* '%s'" % inst)
                return (None, None)
            if not highest or highest < reg[prime]:
                highest = reg[prime]
    # print("%s instructions, %s unique registers" % (icount, rcount))
    return (max(reg.iteritems(), key=operator.itemgetter(1))[1], highest)


TESTDATA = [
    "b inc 5 if a > 1",
    "a inc 1 if b < 5",
    "c dec -10 if a >= 1",
    "c inc -20 if c == 10",
]

if __name__ == "__main__":
    lines = TESTDATA
    (lg, hg) = largest(lines)
    # print("Test: largest=%s, highest=%s" % (l, h))
    assert(lg == 1)
    assert(hg == 10)

    lines = [line.strip() for line in open("puzzle_data.txt")]
    print("Part 1: %s\nPart 2: %s" % largest(lines))
