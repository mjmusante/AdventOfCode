#! /usr/bin/python

from __future__ import print_function

import re
import sys

CODE = re.compile(r"\((\d+)x(\d+)\)")


def decompress(string):
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


testdata = {
        "ADVENT": "ADVENT",
        "A(1x5)BC": "ABBBBBC",
        "(3x3)XYZ": "XYZXYZXYZ",
        "A(2x2)BCD(2x2)EFG": "ABCBCDEFEFG",
        "(6x1)(1x3)A": "(1x3)A",
        "X(8x2)(3x3)ABCY": "X(3x3)ABC(3x3)ABCY",
    }

for t in testdata:
    l = decompress(t)
    if l != testdata[t]:
        print("For '%s': expecting %s, got %s" % (t, testdata[t], l))
        sys.exit(1)

with open("day09.txt") as f:
    data = f.read().strip()

print(len(decompress(data)))
