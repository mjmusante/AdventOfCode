#! /usr/bin/python

from __future__ import print_function

import md5
import sys

keystr = "abbhdwsy"

door2 = [None, None, None, None, None, None, None, None]

base = md5.new()
base.update(keystr)

print("Code for first door: ", end="")
sys.stdout.flush()

i = 0
digits1 = 0
digits2 = 0

while digits1 < 8 or digits2 < 8:
    c = base.copy()
    c.update("%s" % i)
    i += 1
    result = c.hexdigest()
    if result.startswith("00000"):
        if digits1 < 8:
            digits1 += 1
            print("%s" % result[5], end="")
            sys.stdout.flush()
        if digits2 < 8:
            (pos, val) = result[5:7]
            if pos >= '0' and pos < '8':
                pos = int(pos)
                if door2[pos] == None:
                    door2[pos] = val
                    digits2 += 1

print("\nCode for second door: %s" % "".join(door2))

