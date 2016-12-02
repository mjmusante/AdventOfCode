#! /usr/bin/python

from __future__ import print_function

import sys

TRANS = {
        "U": 0, "D": 1, "L": 2, "R": 3,
    }

TRACK_pt1 = {
        "1": ("1", "4", "1", "2"),
        "2": ("2", "5", "1", "3"),
        "3": ("3", "6", "2", "3"),
        "4": ("1", "7", "4", "5"),
        "5": ("2", "8", "4", "6"),
        "6": ("3", "9", "5", "6"),
        "7": ("4", "7", "7", "8"),
        "8": ("5", "8", "7", "9"),
        "9": ("6", "9", "8", "9"),
    }

TRACK_pt2 = {
        "1": ("1", "3", "1", "1"),
        "2": ("2", "6", "2", "3"),
        "3": ("1", "7", "2", "4"),
        "4": ("4", "8", "3", "4"),
        "5": ("5", "5", "5", "6"),
        "6": ("2", "A", "5", "7"),
        "7": ("3", "B", "6", "8"),
        "8": ("4", "C", "7", "9"),
        "9": ("9", "9", "8", "9"),
        "A": ("6", "A", "A", "B"),
        "B": ("7", "D", "A", "C"),
        "C": ("8", "C", "B", "C"),
        "D": ("B", "D", "D", "D"),
    }

def get_code(start, seq, track):
    for s in seq:
        start = track[start][TRANS[s]]
    return start

def find_code(steps, track):
    pos = "5"
    rslt = ""
    for s in steps:
        pos = get_code(pos, s, track)
        rslt = "%s%s" % (rslt, pos)

    return rslt

test_pt1 = {
        ( "ULL", "RRDDD", "LURDL", "UUUUD", ): "1985"
}

test_pt2 = {
        ( "ULL", "RRDDD", "LURDL", "UUUUD", ): "5DB3"
}

for t in test_pt1:
    ans = find_code(t, TRACK_pt1)
    if ans != test_pt1[t]:
        print("Expecting %s, got %s" % (test_pt1[t], ans))
        sys.exit(1)

for t in test_pt2:
    ans = find_code(t, TRACK_pt2)
    if ans != test_pt2[t]:
        print("Expecting %s, got %s" % (test_pt2[t], ans))
        sys.exit(1)

code = [line.strip() for line in open("day02.txt")]
print("Part 1 code: %s" % find_code(code, TRACK_pt1))
print("Part 2 code: %s" % find_code(code, TRACK_pt2))
