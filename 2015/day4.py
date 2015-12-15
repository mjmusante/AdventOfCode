#! /usr/bin/python

import hashlib

key = "yzbqklnj"

found = [False, False, False, False, False, False]
ary = ["0", "00", "000", "0000", "00000", "000000" ]

num_found = 0

count = -1
while num_found < len(found):
    count += 1
    h = hashlib.md5("%s%s" % (key, count)).hexdigest()
    for i in range(0, len(found)):
        if found[i]:
            continue
        if h.startswith(ary[i]):
            print("First hash starting with %s is number %s" % (ary[i], count))
            num_found += 1
            found[i] = True

if False:
    if not found5 and h.startswith("00000"):
        print("First number with five zeros is %s" % count)
        found5 = True
    if not found6 and h.startswith("000000"):
        print("First number with six zeros is %s" % count)
        found6 = True
    if found5 and found6:
        print("break")
