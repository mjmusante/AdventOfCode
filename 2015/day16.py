#! /usr/bin/python

CLUE = {
        "children": 3,
        "cats": 7,
        "samoyeds": 2,
        "pomeranians": 3,
        "akitas": 0,
        "vizslas": 0,
        "goldfish": 5,
        "trees": 3,
        "cars": 2,
        "perfumes": 1,
    }

with open("day16.txt") as f:
        lines = f.read().splitlines()

def found_match(data):
    global CLUE
    for d in data:
        item, count = d.split(":")
        count = int(count)
        i = item.strip()
        if i == "cats" or i == "trees":
            if CLUE[i] > count:
                return False
        elif i == "pomeranians" or i == "goldfish":
            if CLUE[i] < count:
                return False
        elif CLUE[i] != int(count):
            return False
    return True

for l in lines:
    sue, data = l.split(":", 1)
    data = data.split(",")
    if found_match(data):
        print(sue)
