#! /usr/bin/python

import re
import sys
import itertools

with open("day9.txt") as f:
    lines = f.read().splitlines()

print("Read in %s lines" % len(lines))


#lines = [
#        "London to Dublin = 464",
#        "London to Belfast = 518",
#        "Dublin to Belfast = 141",
#        ]

location = set()
dist = {}

def set_length(s, d, l):
    global dist
    dist["".join(sorted([s, d]))] = l

def length(s, d):
    global dist
    return dist["".join(sorted([s, d]))]

shortest = 0
longest = 0
scanner = re.compile("(.*) to (.*) = (.*)")
for l in lines:
    m = scanner.match(l)
    if not m:
        print("Line failed to match:")
        print(">>%s<<" % l)
        sys.exit(1)

    src = m.group(1)
    dest = m.group(2)
    ly = int(m.group(3))
    shortest += ly

    location.add(src)
    location.add(dest)
    set_length(src, dest, ly)

for path in itertools.permutations(location):
    travel = 0
    prev = None
    for p in path:
        if not prev:
            prev = p
        else:
            travel += length(prev, p)
            prev = p
    if travel < shortest:
        shortest = travel
    if travel > longest:
        longest = travel

print("Shortest path is %s" % shortest)
print("Longest path is %s" % longest)
