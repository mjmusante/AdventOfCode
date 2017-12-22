import re
import sys

SPIN = re.compile(r's(\d+)')
EXCHANGE = re.compile(r'x(\d+)/(\d+)')
PARTNER = re.compile(r'p(.)/(.)')

TESTDATA = "s1,x3/4,pe/b"
FULLSTRING = "abcdefghijklmnop"


def dance(positions, moves):
    for move in moves:
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

            newpos = positions[:p1] + c2 + positions[p1 + 1:]
            positions = newpos[:p2] + c1 + newpos[p2 + 1:]
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


def find_loop(start, moves):
    count = 1
    newpos = dance(start, moves)
    while newpos != start:
        count += 1
        newpos = dance(newpos, moves)
    return count


def get_position_at_cycle(start, moves, num_cycles):
    for i in range(num_cycles % find_loop(start, moves)):
        start = dance(start, moves)

    return start


if __name__ == "__main__":
    moves = TESTDATA.split(",")
    d = dance("abcde", moves)

    if d != "baedc":
        print("Test 1 fail: expecting 'baedc' got '%s'" % d)
        sys.exit(1)

    moves = [line.strip() for line in open("puzzle_data.txt")][0].split(",")
    print("Part 1: %s" % dance(FULLSTRING, moves))

    # print("Cycle count %s" % find_loop(FULLSTRING, moves))
    print("Part 2: %s" %
          get_position_at_cycle(FULLSTRING, moves, 1000000000))
