#! /usr/bin/python

import sys
import re

class Reindeer:
    def __init__(self, name, speed, time, rest):
        self.name = name
        self.speed = speed
        self.time = time
        self.rest = rest
        self.unit = time + rest
        self.points = 0

    def distance(self, sec):
        dist = 0
        if sec > self.unit:
            units = int(sec / self.unit)
            dist = units * self.speed * self.time
            sec %= self.unit
        dist += min(self.time, sec) * self.speed
        return dist

    def add_a_point(self):
        self.points += 1

SCAN = re.compile("(.*) can fly (.*) km/s for (.*) seconds, " \
        "but then must rest for (.*) seconds.")

def race(rules, sec, do_points=False):
    reindeer = []
    for r in rules:
        m = SCAN.match(r)
        if not m:
            print("Parse error:")
            print(r)
            sys.exit(1)
        who = m.group(1)
        speed = int(m.group(2))
        time = int(m.group(3))
        rest = int(m.group(4))
        reindeer.append(Reindeer(who, speed, time, rest))

    if do_points:
        points = 0
        for i in range(0, sec):
            furthest = None
            dist = 0
            for r in reindeer:
                d = r.distance(i + 1)
                if d > dist:
                    dist = d
                    furthest = r
            furthest.add_a_point()
            if points < furthest.points:
                points = furthest.points
        return points

    maxdist = 0
    for r in reindeer:
        d = r.distance(sec)
        if d > maxdist:
            maxdist = d
    return maxdist

test_case="""Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.""".splitlines()

print("For %s sec, max dist is %s" % (1000, race(test_case, 1000)))
print("For %s sec, max points is %s" % (1000, race(test_case, 1000, True)))

with open("day14.txt") as f:
    data = f.read().splitlines()

print("For 2503 sec, max dist is %s" % race(data, 2503))
print("For 2503 sec, max points is %s" % race(data, 2503, True))
