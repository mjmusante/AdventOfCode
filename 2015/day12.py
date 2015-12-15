#! /usr/bin/python

import json
import sys

def sum_nums(j, ignore=None):
    total = 0

    if type(j) == unicode:
        return 0

    if type(j) == int:
        return j

    if type(j) == list:
        for i in j:
            total += sum_nums(i, ignore)
        return total

    if type(j) == dict:
        for i in j:
            if j[i] == ignore:
                return 0
            total += sum_nums(j[i], ignore)
        return total

    print("Unknown type %s" % type(j))
    sys.exit(1)

test_cases = {
        '[1,2,3]' : 6,
        '{"a": { "b": 4 }, "c" : -1 }': 3,
        '{"a" : 2, "b": 4}': 6,
        '[[[3]]]': 3,
        '{"a": [ -1, 1 ]}': 0,
        '[]': 0,
        '{}': 0,
        '["green", [{"a": 77, "c": "yellow", "b": 144}]]': 221,
    }

for n in test_cases:
    try:
        x = json.loads(n)
    except:
        print("Could not parse '%s'" % n)
        sys.exit(1)

    r = sum_nums(x)
    if r != test_cases[n]:
        print("'%s' did not evaluate to %s: got %s instead" %
                (n, test_cases[n], r))
        sys.exit(1)
    # print("TC %s => %s" % (n, r))

red_test_cases = {
        '[1,2,3]': 6,
        '[1,{"c":"red","b":2},3]': 4,
        '{"d":"red","e":[1,2,3,4],"f":5}': 0,
        '[1,"red",5]': 6,
    }

for n in red_test_cases:
    try:
        x = json.loads(n)
    except:
        print("Could not parse %s" % n)
        sys.exit(1)

    r = sum_nums(x, "red")
    if r != red_test_cases[n]:
        print("'%s' did not evaluate to %s: got %s instead" %
                (n, red_test_cases[n], r))
        sys.exit(1)
    print("TC %s => %s" % (n, r))


with open("day12.txt") as f:
    line = f.read()

x = json.loads(line)

print("sum = %s" % sum_nums(x))
print("ignoring red, sum = %s" % sum_nums(x, "red"))
