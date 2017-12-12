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



TESTDATA = [
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

tests = { 6: TESTDATA, 5: TESTDATA2 }

for t in tests:
    link = connect(tests[t])
    orig = link.keys()

    (answer, lst) = visitcount(link, 0)
    left = [x for x in orig if (x not in lst)]
    print(left)
    if answer != t:
        print("Test %d, got %s instead" % (t, answer))
    else:
        print("Validated %s" % t)


lines = [line.strip() for line in open("puzzle_data.txt")]
link = connect(lines)
orig = link.keys()

(part1, lst) = visitcount(link, 0)
print("Part 1: %s" % part1)

left = [x for x in orig if (x not in lst)]
groups = 1

while len(left) > 0:
    orig = left
    (_, newlst) = visitcount(link, left[0])
    groups += 1
    left = [x for x in orig if (x not in newlst)]
print("Part 2: %s" % groups)
