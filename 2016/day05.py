#! /usr/bin/python

from __future__ import print_function

import md5
import sys

keystr = "abbhdwsy"

base = md5.new()
base.update(keystr)

i = 0
digits = 0
while digits < 8:
    c = base.copy()
    c.update("%s" % i)
    i += 1
    result = c.hexdigest()
    if result.startswith("00000"):
        digits += 1
        print("%s" % result[5], end="")
        sys.stdout.flush()
print("")
