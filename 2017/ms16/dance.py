import re
import sys

SPIN = re.compile(r's(\d+)')
EXCHANGE = re.compile(r'x(\d+)/(\d+)')
PARTNER = re.compile(r'p(.)/(.)')

TESTDATA = "s1,x3/4,pe/b"

def dance(positions, moves):
    movelist = moves.split(",")
    for move in movelist:
        m = SPIN.match(move)
        if m:
            s = int(m.group(1))
            positions = positions[-s:] + positions[:-s]
            continue


        m = EXCHANGE.match(move)
        if m:
            p1 = int(m.group(1))
            p2 = int(m.group(2))
            c1 = positions[p1]
            c2 = positions[p2]

            newpos = positions[:p1] + c2 + positions[p1+1:]
            positions = newpos[:p2] + c1 + newpos[p2+1:]
            continue

        m = PARTNER.match(move)
        if m:
            p = positions.replace(m.group(1), ".")
            p = p.replace(m.group(2), m.group(1))
            positions = p.replace(".", m.group(2))
            continue

        print("Error: failed to parse '%s'" % move)
        sys.exit(1)

    return positions

print("Test 1: %s" % dance("abcde", TESTDATA))

line = [line.strip() for line in open("puzzle_data.txt")][0]
print("Part 1: %s" % dance("abcdefghijklmnop", line))
