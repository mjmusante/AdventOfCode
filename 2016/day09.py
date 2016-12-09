#! /usr/bin/python

from __future__ import print_function

import re
import sys

CODE = re.compile(r"\((\d+)x(\d+)\)")


def decompress_v1(string):
    rslt = ""
    i = 0
    while i < len(string):
        if string[i] == "(":
            m = CODE.match(string[i:])
            if not m:
                print("Error parsing %s..." % string[i:i+10])
                sys.exit(1)
            (l, r) = m.group(1, 2)
            i += len(m.group(0))
            substr = string[i:i+int(l)]
            rslt += substr * int(r)
            i += len(substr)
        else:
            rslt += string[i]
            i += 1

    return rslt

def decompress_len(string):
    rslt = 0
    i = 0
    while i < len(string):
        if string[i] == "(":
            m = CODE.match(string[i:])
            if not m:
                print("Error parsing %s..." % string[i:i+10])
                sys.exit(1)
            (l, r) = m.group(1, 2)
            i += len(m.group(0))
            substr = string[i:i+int(l)]
            rslt += decompress_len(substr) * int(r)
            i += int(l)
        else:
            rslt += 1
            i += 1

    return rslt



testdata1 = {
        "ADVENT": "ADVENT",
        "A(1x5)BC": "ABBBBBC",
        "(3x3)XYZ": "XYZXYZXYZ",
        "A(2x2)BCD(2x2)EFG": "ABCBCDEFEFG",
        "(6x1)(1x3)A": "(1x3)A",
        "X(8x2)(3x3)ABCY": "X(3x3)ABC(3x3)ABCY",
    }

testdata2 = {
        "(3x3)XYZ": 9,
        "X(8x2)(3x3)ABCY": 20,
        "(27x12)(20x12)(13x14)(7x10)(1x12)A": 241920,
        "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN": 445,
    }

for t in testdata1:
    l = decompress_v1(t)
    if l != testdata1[t]:
        print("For '%s': expecting %s, got %s" % (t, testdata1[t], l))
        sys.exit(1)

for t in testdata2:
    l = decompress_len(t)
    if l != testdata2[t]:
        print("For '%s': expecting %s, got %s" % (t, testdata2[t], l))
        sys.exit(1)

with open("day09.txt") as f:
    data = f.read().strip()

print("V1 Decompress length: %s" % len(decompress_v1(data)))
print("V2 Decompress length: %s" % decompress_len(data))
