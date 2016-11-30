#! /usr/bin/python

import itertools
from operator import mul

with open("day24.txt") as f:
    lines = f.read().splitlines()

pack = []
for i in lines:
    pack.append(int(i))


print("Total packages %s; sum = %s" % (len(pack), sum(pack)))
portion = sum(pack) / 4
lowest_qe = reduce(mul, pack, 1)

def summer(boxen, amount):
    for i in range(1, len(boxen)):
        for j in itertools.combinations(boxen, i):
            if sum(j) == amount:
                yield j

def remaining(origlist, remlist):
    o = list(origlist)
    for i in remlist:
        o.remove(i)
    return o


if False:
    for i in summer(pack, portion):
        r1 = remaining(pack, i)
        for j in summer(r1, portion):
            r2 = remaining(r1, j)
            if sum(r2) != portion:
                continue
            qe = reduce(mul, i, 1)
            if qe < lowest_qe:
                print("%s / %s / %s" % (i, j, r2))
                print("%s" % reduce(mul, i, 1))
                lowest_qe = qe

for i in summer(pack, portion):
    r1 = remaining(pack, i)
    for j in summer(r1, portion):
        r2 = remaining(r1, j)
        for k in summer(r2, portion):
            r3 = remaining(r2, k)
            if sum(r3) != portion:
                continue
            qe = reduce(mul, i, 1)
            if qe < lowest_qe:
                print("%s / %s / %s / %s" % (i, j, k, r3))
                print("%s" % reduce(mul, i, 1))
                lowest_qe = qe

print(lowest_qe)
