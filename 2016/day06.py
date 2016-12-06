#! /usr/bin/python

from __future__ import print_function

from collections import Counter

import sys


def get_code(cyphertext, first):
    result = ""
    codelen = len(cyphertext[0])    # assumption: all strings are same length
    for i in range(0, codelen):
        calc = Counter([c[i] for c in cyphertext])
        if first:
            let, cnt = calc.most_common(1)[0]
        else:
            let, cnt = calc.most_common()[-1]
        result += let
    return result



testdata = [
    "eedadn",
    "drvtee",
    "eandsr",
    "raavrd",
    "atevrs",
    "tsrnev",
    "sdttsa",
    "rasrtv",
    "nssdts",
    "ntnada",
    "svetve",
    "tesnvt",
    "vntsnd",
    "vrdear",
    "dvrsen",
    "enarar",
]


if get_code(testdata, first=True) != "easter":
    print("oops - easter busticated")
    sys.exit(1)
if get_code(testdata, first=False) != "advent":
    print("oops - advent busticated")
    sys.exit(1)

cyphertext = [line.strip() for line in open("day06.txt")]

print(" Most frequent: %s" % get_code(cyphertext, first=True))
print("Least frequent: %s" % get_code(cyphertext, first=False))
