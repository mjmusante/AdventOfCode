#! /usr/bin/python

import itertools
import re
import sys

SCAN = re.compile("(.*) would (.*) (.*) happiness units by sitting next to (.*).")

test_case = """Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
"""

def max_happiness(s, addme=False):
    global SCAN
    seating = {}
    attendee = []

    if addme:
        attendee=["you"]

    for l in s:
        m = SCAN.match(l)
        if m:
            pair = "%s%s" % (m.group(1), m.group(4))
            guest = m.group(1)
            if guest not in attendee:
                attendee.append(guest)
                if addme:
                    seating["you%s" % guest] = 0
                    seating["%syou" % guest] = 0
            value = int(m.group(3))
            if m.group(2) == "gain":
                seating[pair] = value
            else:
                seating[pair] = -value
        else:
            print("Could not parse:")
            print(l)
            sys.exit(1)

    print("Working with %s attendees" % len(attendee))
    maxhap = 0
    for chart in itertools.permutations(attendee):
        haps = 0
        for i in range(0, len(attendee)):
            consider = chart[i]
            pair1 = "%s%s" % (consider, chart[i - 1])
            if i + 1 == len(attendee):
                pair2 = "%s%s" % (consider, chart[0])
            else:
                pair2 = "%s%s" % (consider, chart[i + 1])
            # print("pair1=%s, pair2=%s" % (seating[pair1], seating[pair2]))
            haps += seating[pair1] + seating[pair2]
        if haps > maxhap:
            # print("going from %s to %s" % (maxhap, haps))
            maxhap = haps

    return maxhap

tc = test_case.splitlines()
r = max_happiness(tc)
if r != 330:
    print("Expected 330 but got %s" % r)
    sys.exit(1)

with open("day13.txt") as f:
    data = f.read().splitlines()

print("Day 13 maximum hapiness: %s" % max_happiness(data))
print("Adding yourself: %s" % max_happiness(data, True))
