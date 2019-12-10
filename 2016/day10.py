#! /usr/bin/python

from __future__ import print_function

import re
import sys

CMD = re.compile(r"bot (\d+) gives low to (\S+) (\d+) and high to (\S+) (\d+)")
LOAD = re.compile(r"value (\d+) goes to bot (\d+)")


class Factory:
    def __init__(self):
        self.botlist = dict()
        self.output = dict()


    def show_output(self):
        print("part 2 = %s" % (int(self.output["0"][0]) * int(self.output["1"][0]) * int(self.output["2"][0])))


    def add(self, bot, low, low_dest, high, high_dest):
        if bot in self.botlist:
            b = self.botlist[bot]
        else:
            b = Bot()
            self.botlist[bot] = b

        if b.progged:
            print("bot %s is getting more than one directive" % bot)
            sys.exit(1)
        b.prog(low, low_dest, high, high_dest)

    def load(self, bot, val):
        if bot not in self.botlist:
            self.botlist[bot] = Bot()

        b = self.botlist[bot]

        # print("loading bot %s with val-%s" % (bot, val))
        if b.give(val):
            # l/h - low, high
            # t/b/v - type, bot, value
            (lt, lb, lv, ht, hb, hv) = b.unload()
            if lv == 17 and hv == 61:
                print("part 1 = %s" % bot)

            # print("bot %s giving val-%s to %s %s and val-%s to %s %s" %
            #         (bot, lv, lt, lb, hv, ht, hb))
            self.handle(lt, lb, lv)
            self.handle(ht, hb, hv)

    def handle(self, outtype, bot, value):
        if outtype == 'output':
            if bot not in self.output:
                self.output[bot] = list()
            self.output[bot].append(value)
            # print("Output %s getting val %s" % (bot, value))
        else:
            self.load(bot, value)


class Bot:
    def __init__(self):
        self.holding = list()
        self.progged = False

    def prog(self, low, low_dest, high, high_dest):
        self.low = low
        self.low_dest = low_dest
        self.high = high
        self.high_dest = high_dest
        self.progged = True

    def give(self, val):
        self.holding.append(int(val))
        # print(self.holding)
        return len(self.holding) > 1

    def unload(self):
        if self.holding[0] < self.holding[1]:
            (low, high) = self.holding
        else:
            (high, low) = self.holding
        assert(low <= high)
        self.holding = list()
        return (self.low, self.low_dest, low, self.high, self.high_dest, high)




def process(cmds):
    f = Factory()
    loader = list()

    for c in cmds:
        m = CMD.match(c)
        if m:
             f.add(*(m.group(1, 2, 3, 4, 5)))
             continue
        m = LOAD.match(c)
        if not m:
            print("'%s' does not match CMD or LOAD" % c)
            sys.exit(1)
        loader.append([m.group(2), m.group(1)])

    for l in loader:
        f.load(l[0], l[1])

    f.show_output()


testdata = [
        "value 5 goes to bot 2",
        "bot 2 gives low to bot 1 and high to bot 0",
        "value 3 goes to bot 1",
        "bot 1 gives low to output 1 and high to bot 0",
        "bot 0 gives low to output 2 and high to output 0",
        "value 2 goes to bot 2",
    ]

# process(testdata)

inst = [line.strip() for line in open("day10.txt")]

process(inst)
