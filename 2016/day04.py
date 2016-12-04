#! /usr/bin/python

from __future__ import print_function

import collections
import re
import string
import sys

def get_code(codestr):
    count = collections.defaultdict(int)
    for s in "".join(sorted(codestr)):
        count[s] += 1

    def cmp(a, b):
        if count[a] == count[b]:
            if a < b:
                return -1
            elif a > b:
                return 1
            return 0
        return count[b] - count[a]

    return "".join(sorted(count, cmp=cmp)[:5])

ALPHABET = "abcdefghijklmnopqrstuvwxyz"
EXT = re.compile("(\d+)\[(.*)\]")

lines = [line.strip() for line in open("day04.txt")]

total = 0
for l in lines:
    g = l.split("-")
    codestr = "-".join(g[:-1])
    m = EXT.match(g[-1])
    (code, chars) = (int(m.group(1)), m.group(2))
    if chars == get_code("".join(g[:-1])):
        total += code

        rot = code % 26
        if rot == 0:
            print(codestr)
        else:
            xlate = string.maketrans(ALPHABET + "-",
                    ALPHABET[rot:] + ALPHABET[:rot] + " ")
            if string.translate(codestr, xlate) == "northpole object storage":
                print("Object storage sector id: %s" % code)


print("sector ID sum: %s" % total)
