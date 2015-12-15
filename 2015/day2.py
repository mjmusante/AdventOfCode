#! /usr/bin/python

box = []

with open("day2.txt") as f:
    box = f.read().splitlines()

print("boxes = %s" % len(box))
sqft = 0
length = 0
for b in box:
    d = b.split("x")
    l, w, h = int(d[0]), int(d[1]), int(d[2])
    side1 = l * w
    side2 = l * h
    side3 = w * h
    sqft += 2 * side1 + 2 * side2 + 2 * side3 + min(side1, min(side2, side3))
    p1, p2, p3 = sorted((l, w, h))
    length += p1 * 2 + p2 * 2 + l * w * h

print("Total square feet needed: %s" % sqft)
print("Total ribbon length: %s" % length)
