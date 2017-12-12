import re

SCAN = re.compile("^(\d+) <-> (.*)$")

def visitcount(link, start):
    visited = dict()
    visited[start] = 1

    def visitlink(link, start):
        count = 1
        visited[start] = 1
        for x in link[start]:
            if x not in visited:
                count += visitlink(link, x)
        return count

    return (visitlink(link, start), visited.keys())

def groups(link):
    gcount = 1
    
    # first group starts with id 0
    (pcount, linked) = visitcount(link, 0)

    # find the list of ids that have not been visited
    remain = [x for x in link.keys() if (x not in linked)]

    # while there are remaining ids, start with the first of those
    while len(remain) > 0:
        gcount += 1
        (_, foo) = visitcount(link, remain[0])
        remain = [x for x in remain if (x not in foo)]

    return (pcount, gcount)


def connect(ary):
    link = dict()
    for l in ary:
        m = SCAN.match(l)
        if not m:
            print("fail at '%s'" % l)
            return 0
        prg = int(m.group(1))
        if prg not in link:
            link[prg] = []
        for p in m.group(2).split(", "):
            p2 = int(p)
            if p2 not in link:
                link[p2] = []
            link[prg].append(p2)
    return link



TESTDATA1 = [
    "0 <-> 2",
    "1 <-> 1",
    "2 <-> 0, 3, 4",
    "3 <-> 2, 4",
    "4 <-> 2, 3, 6",
    "5 <-> 6",
    "6 <-> 4, 5",
]

TESTDATA2 = [
    "0 <-> 480, 1750",
    "480 <-> 0, 314",
    "314 <-> 480, 1676",
    "1750 <-> 0",
    "1676 <-> 314",
]

TESTDATA3 = [
    "0 <-> 0",
    "3 <-> 3",
    "31 <-> 31",
    "314 <-> 314",
    "3141 <-> 3141",
    "31415 <-> 31415",
    "314159 <-> 314159",
]

tests = { (6, 2): TESTDATA1, (5, 1): TESTDATA2, (1, 7): TESTDATA3 }

for t in tests:
    link = connect(tests[t])

    rslt = groups(link)
    if rslt != t:
        print("Test fail: expecting %s, got %s" % (t, rslt))


lines = [line.strip() for line in open("puzzle_data.txt")]
link = connect(lines)

print("Part 1: %s\nPart 2: %s" % groups(link))
