import re

SCAN = re.compile(r'(\d+): (\d+)$')

def decode_layers(lines):
    layer = dict()
    for l in lines:
        m = SCAN.match(l)
        if not m:
            print("parse fail on '%s'" % l)
            return 0
        idx = int(m.group(1))
        if idx in layer:
            print("duplicate def: '%s'" % l)
            return 0
        layer[idx] = int(m.group(2))
    return layer

def sev(layer, delay=0):
    hit = False
    last = max(layer.keys())
    stage = 0
    severity = 0
    while stage <= last:
        if stage in layer:
            psec = (delay + stage)
            mod = 2 * (layer[stage] - 1)
            if (stage + delay) % mod == 0:
                severity += stage * layer[stage]
                # short-circuit when searching for delay
                if delay > 0:
                    return (0, True)
                hit = True
        stage += 1
    return (severity, hit)

def get_delay(layer):
    delay = 0
    (_, hit) = sev(layer, delay)
    while hit:
        delay += 1
        (_, hit) = sev(layer, delay)
    return delay

TESTDATA = [
    "0: 3",
    "1: 2",
    "4: 4",
    "6: 4",
]

l = decode_layers(TESTDATA)
print("Test: severity = %s; delay = %s" % (sev(l)[0], get_delay(l)))

puzzle = decode_layers([line.strip() for line in open("puzzle_data.txt")])
print("Part 1: %s" % sev(puzzle)[0])
print("Part 2: %s" % get_delay(puzzle))
