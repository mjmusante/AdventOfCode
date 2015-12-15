#! /usr/bin/python

import re
import sys

with open("day7.txt") as f:
    lines = f.read().splitlines()

test = [
        "123 -> x",
        "456 -> y",
        "x AND y -> d",
        "x OR y -> e",
        "x LSHIFT 2 -> f",
        "y RSHIFT 2 -> g",
        "NOT x -> h",
        "NOT y -> i",
        ]

instructions = lines

print("Read in %s instructions" % len(instructions))

def get_value_of(connection, wire):
    if wire[0] >= "0" and wire[0] <= "9":
        return int(wire)
    if wire not in connection:
        print("Cannot find value of %s in connection list" % wire)
        sys.exit(1)
    x = connection[wire]
    if type(x) == int:
        return x
    if x.startswith("NOT "):
        connection[wire] = ~get_value_of(connection, x[4:])
    else:
        foo = x.split(" ")
        if len(foo) == 1:
            return get_value_of(connection, x)
        elif len(foo) != 3:
            print("can't split %s properly" % x)
            sys.exit(1)
        left = get_value_of(connection, foo[0])
        right = get_value_of(connection, foo[2])
        if foo[1] == "AND":
            connection[wire] = left & right
        elif foo[1] == "OR":
            connection[wire] = left | right
        elif foo[1] == "LSHIFT":
            connection[wire] = (left << right) & 0xffff
        elif foo[1] == "RSHIFT":
            connection[wire] = left >> right
        else:
            print("Unknown directive for wire %s: %s" % (wire, x))
            sys.exit(1)
    while connection[wire] < 0:
        connection[wire] += 65536
    return connection[wire]


def set_up_connections(inst):
    connection = {}
    m = re.compile("^(.*) -> ([a-z]+)$")
    for i in inst:
        q = m.search(i)
        if not q:
            print("Line fails to match: %s" % i)
            sys.exit(1)
        wire = q.group(2)
        if wire in connection:
            print("Wire %s has multiple connections at '%s'" % (wire, i))
            sys.exit(1)
        connection[wire] = q.group(1)
    return connection

c1 = set_up_connections(instructions)
val_a = get_value_of(c1, "a")
print("Signal on wire a: %s" % val_a)

c2 = set_up_connections(instructions)
c2["b"] = val_a
print("New signal on wire a: %s" % get_value_of(c2, "a"))
