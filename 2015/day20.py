#! /usr/bin/python

# value = 29000000

from __future__ import print_function
from math import sqrt

import sys

def factors(n):    
    return set(reduce(list.__add__, 
        ([i, n//i] for i in range(1, int(sqrt(n)) + 1) if n % i == 0)))

# for i in range(1, 10):
#     print(sum(factors(i)))

print("Part 2:")

# we know that 637560 is "too low" (because this code was buggy)
for i in range(637560, 1000000):
    tot = 0
    for j in sorted(factors(i), reverse=True):
        if i / j > 50:
            break
        tot += j * 11
    if tot < 29000000:
        continue
    print("%s gets %s" % (i, tot))
    break

print("Part 1:")

for i in range(100000, 1000000):
    if sum(factors(i)) < 2900000:
        continue
    print(i)
    break

sys.exit(1)



def presents_in_house(house):
    total = 0
    for elf in range(1, house + 1):
        if house % elf == 0:
            total += 10 * elf
    return total

house = 1
total = 0
most = 0
while total < 29000000:
    house *= 2
    total = presents_in_house(house)
    print("house %s, total %s" % (house, total))

# binary chop
lowest = house / 2
highest = house

while True:
    mid = (highest + lowest) / 2
    presents = presents_in_house(mid)
    print("(%s,%s,%s), total = %s" % (lowest, mid, highest, presents))
    if presents < 29000000:
        lowest = mid + 1
    elif presents > 29000000:
        highest = mid - 1
    if lowest >= highest:
        print("no exact match found")
        break
