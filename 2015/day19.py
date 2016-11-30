#! /usr/bin/python

import re
import sys

CONVERSION = re.compile("^(.*) => (.*)$")

with open("day19.txt") as f:
    lines = f.read().splitlines()

test = """e => H
e => O
H => HO
H => OH
O => HH

HOH""".splitlines()

test2 = """e => OF
e => FO
F => MgAr
F => Ar
O => AuAg
Mg => Au
Mg => Ag
Ar => Tn
Ar => Pb
Au => Ag
Ag => Au
Tn => F
Pb => F

AuAgPb""".splitlines()

data = lines

rule = {}
for l in data:
    m = CONVERSION.match(l)
    if m:
        src = m.group(1)
        dst = m.group(2)
        if src not in rule:
            rule[src] = set()
        rule[src].add(dst)
    elif len(l) > 1:
        compound = l

def one_sub(molecule):
    global rule

    result = set()
    for r in rule:
        l = [m.start() for m in re.finditer(r, molecule)]
        for t in l:
            u = len(r)
            for c in rule[r]:
                front = molecule[0:t]
                back = molecule[t+u:]
                n = front + c + back
                result.add(n)
    return result

print("Total substitutions %s" % len(one_sub(compound)))

found = 1000000000
high = 0
checked = set()

def find_medicine(current, count, search):
    global found, high, checked

    if count == 0:
        checked = set()

    count += 1
    if high < count:
        high = count
    for s in one_sub(current):
        if s in checked:
            continue
        checked.add(s)
        if s == search:
            if count < found:
                found = count
            return count
        if len(s) <= len(search):
            find_medicine(s, count, search)
    return -1

def medicine_insight(molecule):
    # 1. split into elements
    # 2. count elements (telm)
    # 3. count all Rn's and Ar's (tparen)
    # 4. count all Y's (tcommas)
    # 5. return telm - tparen - tcommas * 2 - 1
    return 195

x = find_medicine("e", 0, compound)
print("x %s, found %s, depth %s" % (x, found, high))
