#! /usr/bin/python

from __future__ import print_function

import collections
import re
import string
import sys

def get_code(string):
    count = collections.defaultdict(int)
    for s in string:
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

EXT = re.compile("(\d+)\[(.*)\]")

lines = [line.strip() for line in open("day04.txt")]

total = 0
for l in lines:
    g = l.split("-")
    string = "".join(sorted("".join(g[:-1])))
    m = EXT.match(g[-1])
    (code, chars) = (int(m.group(1)), m.group(2))
    if chars == get_code(string):
        total += code

print(total)
