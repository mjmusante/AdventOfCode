#! /usr/bin/python

from __future__ import print_function

import re


RECT = re.compile(r"(\d+)x(\d+)")
ROTATE = re.compile(r"(\S+) .=(\d+) by (\d+)")

class Display:
    def __init__(self, x, y):
        self.x = x
        self.y = y
        self.disp = list()
        for i in range(0, y):
            self.disp.append(0)


    def mask(self, size):
        return (2 ** size) - 1


    def set_rect(self, w, h):
        bitmask = self.mask(w)
        for i in range(0, h):
            self.disp[i] |= bitmask


    def rotate_row(self, r, dist):
        dist %= self.x
        msbs = self.disp[r] >> (self.x - dist)
        self.disp[r] = (self.disp[r] << dist) & self.mask(self.x)
        self.disp[r] |= msbs


    def rotate_col(self, c, dist):
        dist %= self.y
        mask = ((2 ** self.x) - 1) - (2 ** c)
        a = list()
        for i in range(0, self.y):
            a.append(self.disp[i] & (~mask))
            self.disp[i] &= mask
        for i in range(0, self.y):
            self.disp[(i + dist) % self.y] |= a[i]


    def pixel_count(self):

        def bits_set(num):
            c = 0
            while num != 0:
                num &= (num - 1)
                c += 1
            return c

        count = 0
        for i in range(0, self.y):
            count += bits_set(self.disp[i])
        return count


    def output(self):
        ch = ('.', '#')
        for i in range(0, self.y):
            for j in range(0, self.x):
                print(ch[(self.disp[i] >> j) & 1], end="")
            print("")


g = Display(50, 6)


inst = [line.strip() for line in open("day08.txt")]
for i in inst:
    (cmd, op) = i.split(" ", 1)
    if cmd == "rect":
        m = RECT.match(op)
        g.set_rect(int(m.group(1)), int(m.group(2)))
    else:
        m = ROTATE.match(op)
        if m.group(1) == "row":
            g.rotate_row(int(m.group(2)), int(m.group(3)))
        else:
            g.rotate_col(int(m.group(2)), int(m.group(3)))
print(g.pixel_count())
