
import re
import sys


class SpaceMap:

    def __init__(self, minval=0, maxval=sys.maxint):
        self.minval = minval
        self.maxval = maxval
        self.valid = [(minval, maxval)]

    def in_map(self, val):
        return val >= self.minval and val <= self.maxval


if __name__ == "__main__":
    lines = [l.strip().split("-") for l in open("day20.txt")]
    print("lines = %s" % len(lines))
    lines.sort(key=lambda x: int(x[0]))
    print("%s" % lines[0])
    lowest = 0
    count = 0
    for l in lines:
        (low, high) = (int(l[0]), int(l[1]))
        if lowest >= low and lowest <= high:
            lowest = high + 1
        elif lowest < low:
            count += low - lowest
            lowest = high + 1
        # if lowest >= low and lowest <= high:
        #     lowest = high + 1
        # elif lowest < low:
        #     break
    # print("lowest = %s" % lowest)
    print("count = %s" % count)
