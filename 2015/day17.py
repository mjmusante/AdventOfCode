#! /usr/bin/python

import itertools

test_data = [20, 15, 10, 5, 5]

def fill(amount, containers):
    success = 0
    minlist = {}
    for i in range(0, len(containers)):
        for j in itertools.combinations(containers, i + 1):
            if sum(j) == amount:
                success += 1
                if len(j) in minlist:
                    minlist[len(j)] += 1
                else:
                    minlist[len(j)] = 1
    smallest = sorted(minlist)[0]
    return success, minlist[smallest]

print(fill(25, test_data))

with open("day17.txt") as f:
    lines = f.read().splitlines()

val = []
for i in lines:
    val.append(int(i))
print(fill(150, val))
