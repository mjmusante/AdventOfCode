
import sys

class SpaceMap:
    def __init__(self, minval=0, maxval=sys.maxint):
        self.minval = minval
        self.maxval = maxval
        self.valid = [(minval, maxval)]

    def in_map(self, val):
        return val >= self.minval and val <= self.maxval

    def remove_range(self, low, high):
        for v in self.valid:
            if v[0] == low:
                if v[1] >= high:
                    return
                
            if v[0] > low:
                if v[0] > high:
