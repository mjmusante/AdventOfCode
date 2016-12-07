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

def supports_ssl(seq):
    slist = list()
    hlist = list()

    while True:
        m = PAIR.search(seq)
        if m:
            (snet, hnet) = m.group(1, 2)
            slist.append(snet)
            hlist.append(hnet)
            seq = seq[len(m.group(0)):]
        else:
            slist.append(seq)
            break
    
    for s in slist:
        for i in range(len(s) - 2):
            if s[i] != s[i + 1] and s[i] == s[i + 2]:
                sub = s[i + 1] + s[i] + s[i + 1]
                for h in hlist:
                    if sub in h:
                        return True

    return False


test_tls = {
        "abba[mnop]qrst": True,
        "abcd[bddb]xyyx": False,
        "aaaa[qwer]tyui": False,
        "ioxxoj[asdfgh]zxcvbn": True,
    }

for addr in test_tls:
    rslt = supports_tls(addr)
    if rslt != test_tls[addr]:
        print("Error: %s returned %s but expecting %s" %
                (addr, rslt, test_tls[addr]))
        sys.exit(1)

test_ssl = {
        "aba[bab]xyz": True,
        "xyx[xyx]xyx": False,
        "aaa[kek]eke": True,
        "zazbz[bzb]cdb": True,
    }

for addr in test_ssl:
    rslt = supports_ssl(addr)
    if rslt != test_ssl[addr]:
        print("Error: %s returned %s but expecting %s" %
                (addr, rslt, test_ssl[addr]))
        sys.exit(1)


addrs = [line.strip() for line in open("day07.txt")]

count = 0
for a in addrs:
    if supports_tls(a):
        count += 1
print("%s IPs support TLS" % count)

count = 0
for a in addrs:
    if supports_ssl(a):
        count += 1
print("%s IPs support SSL" % count)
