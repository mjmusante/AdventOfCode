#! /usr/bin/python

from __future__ import print_function

import sys

def is_triangle(s1, s2, s3):
    if s1 + s2 <= s3:
        return False
    if s1 + s3 <= s2:
        return False
    if s2 + s3 <= s1:
        return False
    return True

lines = [line.strip() for line in open("day03.txt")]

possible = 0
for l in lines:
    (x, y, z) = l.split()
    if not is_triangle(int(x), int(y), int(z)):
        continue
    possible += 1
print(possible)

possible = 0
tri = list()
point = 0
for l in lines:
    (x, y, z) = l.split()
    tri.append((int(x), int(y), int(z)),)

    point += 1
    if point == 3:
        while point > 0:
            point -= 1
            if is_triangle(tri[0][point], tri[1][point], tri[2][point]):
                possible += 1
        tri = list()
print(possible)
