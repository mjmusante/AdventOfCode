#! /usr/bin/python

import re
import sys

through = re.compile(" (\d+),(\d+) through (\d+),(\d+)")

with open("day6.txt") as f:
    instructions = f.read().splitlines()

print("Read in %s instructions" % len(instructions))

foo = [0] * 1000
bar = [[0 for i in range(1000)] for i in range(1000)]
total_brightness = 0

for i in instructions:
    m = through.search(i)
    if not m:
        print("Failed to match: %s" % i)
        sys.exit(1)
    start = int(m.group(1))
    end = int(m.group(3))
    left = int(m.group(4))
    right = int(m.group(2))
    seq = 2 ** (left + 1) - 1
    seq &= ~(2 ** right - 1)
    # print("%s,%s,%s" % (start, end, seq))
    if i.startswith("turn on "):
        for n in range(start, end + 1):
            foo[n] |= seq
            for m in range(right, left + 1):
                bar[n][m] += 1
                total_brightness += 1
    elif i.startswith("turn off "):
        for n in range(start, end + 1):
            foo[n] &= ~seq
            for m in range(right, left + 1):
                if bar[n][m] > 0:
                    bar[n][m] -= 1
                    total_brightness -= 1
    elif i.startswith("toggle "):
        for n in range(start, end + 1):
            foo[n] ^= seq
            for m in range(right, left + 1):
                bar[n][m] += 2
                total_brightness += 2
    else:
        print("unknown command: %s" % i)
        sys.exit(1)

def count_bits(val):
    result = 0
    while val:
        val &= val - 1
        result += 1
    return result

count = 0
for n in foo:
    count += count_bits(n)

print("Number of lights on: %s" % count)
print("Total brightness: %s" % total_brightness)
