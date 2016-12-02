#! /usr/bin/python

from __future__ import print_function

import sys

TRANS = {
        "U": 0, "D": 1, "L": 2, "R": 3,
    }

TRACK = {
        1: (1, 4, 1, 2),
        2: (2, 5, 1, 3),
        3: (3, 6, 2, 3),
        4: (1, 7, 4, 5),
        5: (2, 8, 4, 6),
        6: (3, 9, 5, 6),
        7: (4, 7, 7, 8),
        8: (5, 8, 7, 9),
        9: (6, 9, 8, 9),
    }


def get_code(start, seq):
    for s in seq:
        start = TRACK[start][TRANS[s]]
    return start

def find_code(steps):
    pos = 5
    rslt = ""
    for s in steps:
        pos = get_code(pos, s)
        rslt = "%s%s" % (rslt, pos)

    return int(rslt)

tests = {
        ( "ULL", "RRDDD", "LURDL", "UUUUD", ): 1985
}

for t in tests:
    ans = find_code(t)
    if ans != tests[t]:
        print("Expecting %s, got %s" % (tests[t], ans))
        sys.exit(1)

code = [line.strip() for line in open("day02.txt")]
print("Code: %s" % find_code(code))
