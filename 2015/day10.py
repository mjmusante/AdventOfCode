#! /usr/bin/python

import sys

def say_string(s):
    result = ""
    count = 0
    last = None
    for c in s:
        if not last:
            last = c
            count = 1
            continue
        else:
            if last == c:
                count += 1
            else:
                result += "%s%s" % (count, last)
                count = 1
                last = c
    if count > 0:
        result += "%s%s" % (count, last)

    return result

for i in ["1", "11", "21", "1211", "111221"]:
    print("%s -> %s" % (i, say_string(i)))

num = "3113322113"
for i in range(0, 40):
    num = say_string(num)

print("After 40 iterations, 3113322113 becomes a number with length: %s" 
        % len(num))

for i in range(0, 10):
    num = say_string(num)

print("After 50 iterations, 3113322113 becomes a number with length: %s" 
        % len(num))

