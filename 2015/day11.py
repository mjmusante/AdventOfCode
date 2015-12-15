#! /usr/bin/python

from __future__ import print_function

import sys
import re

INVALID = re.compile("[iol]")

def inc_string(s):
    last = s[-1]
    first = s[0:-1]
    if last == "z":
        return inc_string(first) + "a"
    return first + chr(ord(last) + 1)

def is_valid(s):
    # 1/ Passwords may not contain the letters i, o, or l, as
    # these letters can be mistaken for other characters and
    # are therefore confusing.
    global INVALID
    if INVALID.search(s):
        # print("%s: invalid because it contains i, o, or l" % s)
        return False

    # 2/ Passwords must include one increasing straight of
    # at least three letters, like abc, bcd, cde, and so
    # on, up to xyz. They cannot skip letters; abd doesn't count.
    #
    # 3/ Passwords must contain at least two different,
    # non-overlapping pairs of letters, like aa, bb, or zz.

    last_seq = 0
    seq_count = 0
    last_pair1 = 0
    last_pair2 = 0
    found_seq = False
    found_pair1 = False
    found_pair2 = False

    for c in s:
        v = ord(c)
        if not found_seq:
            if last_seq + 1 == v:
                seq_count += 1
                last_seq = v
                if seq_count >= 3:
                    found_seq = True
            else:
                last_seq = v
                seq_count = 1
        if not found_pair1:
            if last_pair1 == v:
                found_pair1 = True
            else:
                last_pair1 = v
        elif not found_pair2 and v != last_pair1:
            if last_pair2 == v:
                found_pair2 = True
            else:
                last_pair2 = v
        if found_seq and found_pair1 and found_pair2:
            return True

    return False

    print("%s invalid because:" % s, end="")
    if not found_seq:
        print(" no three-letter straight", end="")
    if not found_pair1:
        print(" no pairs of letters", end="")
    elif not found_pair2:
        print(" only one pair of letters", end="")
    print("")

# for i in ["a", "bb", "hz", "bzzzz", "aosijfd"]:
    # print("%s -> %s" % (i, inc_string(i)))


passwd = inc_string("vzbxkghb")
while not is_valid(passwd):
   passwd = inc_string(passwd)

print("After vzbxkghb comes %s" % passwd)
again = passwd
passwd = inc_string(passwd)
while not is_valid(passwd):
   passwd = inc_string(passwd)
print("After %s comes %s" % (again, passwd))
