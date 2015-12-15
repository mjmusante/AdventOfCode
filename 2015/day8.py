#! /usr/bin/python

import re
import sys

with open("day8.txt") as f:
    lines = f.read().splitlines()

print("Read in %s lines" % len(lines))

def len_of_string(s):
    in_string = False
    in_escape = False
    in_hex = False
    prev_hex = None
    final_string = 0
    expanded_string = 0
    for c in s:

        if not in_string:
            if c == "\"":
                in_string = True
                expanded_string += 3
            else:
                print("Invalid char in '''%s'''" % s)
                sys.exit(1)
            continue

        if in_hex:
            if prev_hex:
                final_string += 1
                expanded_string += 3
                prev_hex = None
                in_hex = False
            else:
                prev_hex = c
        elif in_escape:
            if c == "\"" or c == "\\":
                final_string += 1
                expanded_string += 2
                in_escape = False
            elif c == "x":
                in_hex = True
                in_escape = False
        elif c == "\\":
            expanded_string += 2
            in_escape = True
        elif c == "\"":
            expanded_string += 3
            return (final_string, expanded_string)
        else:
            expanded_string += 1
            final_string += 1
    print("Fell off the end of '''%s'''" % s)
    sys.exit(1)

print(len_of_string("\"\""))
print(len_of_string("\"abc\""))
print(len_of_string("\"aaa\\\"aaa\""))
print(len_of_string("\"\\x27\""))

datasum = 0
strsum = 0
expsum = 0
for l in lines:
    datasum += len(l)
    s, e = len_of_string(l)
    strsum += s
    expsum += e

print("Total data storage: %s\n" % datasum)
print("Total string length: %s" % strsum)
print("Difference: %s" % (datasum - strsum))
print("Total expanded length: %s" % expsum)
print("Difference: %s" % (expsum - datasum))
