#! /usr/bin/python

from __future__ import print_function

import sys

NORTH = 0
EAST = 1
SOUTH = 2
WEST = 3

INC = [ 1, 1, -1, -1 ]

def get_distance(path):
    facing = NORTH
    xloc = 0
    yloc = 0
    grid = set()
    twice = None

    steps = path.split(", ")

    for s in steps:
        (c, dist) = (s[0], int(s[1:]))

        if c == ' ' or c == ',':
            continue

        if c == 'L':
            facing -= 1
        elif c == 'R':
            facing += 1

        while facing < 0:
            facing += 4
        while facing > 3:
            facing -= 4

        while dist > 0:
            if facing == NORTH or facing == SOUTH:
                xloc += INC[facing]
            else:
                yloc += INC[facing]
            
            dist -= 1

            if twice == None:
                if (xloc, yloc) in grid:
                    twice = abs(xloc) + abs(yloc)
                else:
                    grid.add((xloc, yloc),)

    return (abs(xloc) + abs(yloc), twice)

tests = {
        "R8, R4, R4, R8": (8, 4),
        "R2, L3": (5, None),
        "R2, R2, R2": (2, None),
        "R5, L5, R5, R3": (12, None),
    }

for key in tests:
    (d, t) = get_distance(key)
    if d != tests[key][0]:
        print("For '%s', expecting %s, got %s" % (key, tests[key][0], d))
        sys.exit(1)
    if t != tests[key][1]:
        print("For '%s', twice should be %s, got %s" % (key, tests[key][1], t))
        sys.exit(1)

with open("day01.txt") as f:
    s = f.read().strip()

print("Distance: %s, visited twice: %s" % get_distance(s))
