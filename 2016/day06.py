#! /usr/bin/python

from __future__ import print_function

from collections import Counter

import sys


def get_code(cyphertext):
    result = ""
    codelen = len(cyphertext[0])    # assumption: all strings are same length
    for i in range(0, codelen):
        let, cnt = Counter([c[i] for c in cyphertext]).most_common(1)[0]
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


if get_code(testdata) != "easter":
    print("oops - busticated")
    sys.exit(1)

print("message: %s" % get_code([line.strip() for line in open("day06.txt")]))
