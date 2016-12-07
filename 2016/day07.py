#! /usr/bin/python

from __future__ import print_function

import re
import sys

PAIR = re.compile("([a-z]*)\[([a-z]*)\]")

def contains_pair(seq):
    for i in range(len(seq) - 3):
        sub = seq[i:i+2]
        if seq[i+2:i+4] == sub[::-1] and sub[0] != sub[1]:
            return True
    return False

def supports_tls(seq):
    possible = False
    while True:
        m = PAIR.search(seq)
        if m:
            if contains_pair(m.group(2)):
                return False
            if not possible and contains_pair(m.group(1)):
                possible = True
            seq = seq[len(m.group(0)):]
            if seq == "":
                return possible
        else:
            return possible or contains_pair(seq)

testdata = {
        "abba[mnop]qrst": True,
        "abcd[bddb]xyyx": False,
        "aaaa[qwer]tyui": False,
        "ioxxoj[asdfgh]zxcvbn": True,
    }

for addr in testdata:
    rslt = supports_tls(addr)
    if rslt != testdata[addr]:
        print("Error: %s returned %s but expecting %s" %
                (addr, rslt, testdata[addr]))
        sys.exit(1)


addrs = [line.strip() for line in open("day07.txt")]
count = 0
for a in addrs:
    if supports_tls(a):
        count += 1

print("%s IPs support TLS" % count)
