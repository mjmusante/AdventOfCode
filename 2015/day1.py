#! /usr/bin/python

with open("day1.txt", "r") as f:
    x = f.read().strip()

floor = 0
pos = 0
basement = -1
for c in x:
    pos += 1
    if c == "(":
        floor += 1
    elif c == ")":
        floor -= 1
        if floor < 0 and basement < 0:
            basement = pos
    else:
        print("bad char %s" % c)

print("Final floor: %s" % floor)
print("First basement: %s" % basement)
