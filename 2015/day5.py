#! /usr/bin/python

import re
import sys

naughty = []

def is_nice(s):
    global naughty

    for n in naughty:
        if n.search(s):
            # print("%s : has a naughty string" % s)
            return False

    vowels = 0
    double = 0
    last_c = ""
    for c in s:
        if c == last_c:
            double += 1
        last_c = c
        if c in [ 'a', 'e', 'i', 'o', 'u' ]:
            vowels += 1

    # print("%s : double=%s, vowel=%s" % (s, double, vowels))
    return double > 0 and vowels >= 3

def is_really_nice(s):
    twin = False
    pair = False
    for i in range(0, len(s) - 2):
        if not twin:
            c = s[i:i+2]
            # print(c)
            for j in range(i + 2, len(s) - 1):
                if c == s[j:j+2]:
                    # print("bongo")
                    twin = True
                    break
        if not pair:
            # print("%s/%s" % (s[i], s[i+2]))
            if s[i] == s[i+2]:
                pair = True
                # print("bingo")
        if pair and twin:
            return True
    return False

with open("day5.txt") as f:
    string = f.read().splitlines()

print("Read in %s strings" % len(string))

# convert naughty strings to re's
for n in  [ "ab", "cd", "pq", "xy" ]:
    naughty.append(re.compile(n))

nice = 0
really_nice = 0
for s in string:
    if is_nice(s):
        nice += 1
    if is_really_nice(s):
        really_nice+= 1

print("Number of nice strings: %s" % nice)
print("Number of really nice strings: %s" % really_nice)
