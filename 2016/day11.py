#! /usr/bin/python

from __future__ import print_function

import copy
import itertools
import re
import sys

class Thingy:
    genchip = ['gen', 'chip', ]
    gc = ['g', 'c', ]

    def __init__(self, elmt, ischip):
        self.elmt = elmt
        self.ischip = ischip

    def __str__(self):
        return "%s-%s" % (self.elmt, self.genchip[self.ischip])

    def __repr__(self):
        return self.__str__()

    def __eq__(self, other):
        return self.elmt == other.elmt and self.ischip == other.ischip

    def __ne__(self, other):
        return not self.__eq__(other)

    def get_gen(self):
        return Thingy(self.elmt, False)

    def get_chip(self):
        return Thingy(self.elmt, True)

    def short(self):
        return ("%2.2s" % self.elmt, self.gc[self.ischip])

class Factory:
    def __init__(self):
        self.lift = 0
        self.floor = [list(), list(), list(), list()]

    def __eq__(self, other):
        if self.lift != other.lift:
            return False
        for i in range(0, 4):
            if self.floor[i] != other.floor[i]:
                return False
        return True

    def __ne__(self, other):
        return not self.__eq__(other)

    def place_chip(self, name, floor):
        self.floor[floor].append(Thingy(name, True))

    def place_gen(self, name, floor):
        self.floor[floor].append(Thingy(name, False))

    def printcompact(self, indent=0):
        print(" " * indent, end="")
        for i in [3, 2, 1, 0]:
            print("{%s" %  i, end="")
            if i == self.lift:
                print("E:", end="")
            else:
                print(":", end="")
            for j in self.floor[i]:
                print(" %2.2s-%s" % j.short(), end="")
            print("}", end="")
        print("")

    def printstate(self, indent=0):
        for i in [3, 2, 1, 0]:
            print("%s%s:" % (" " * indent, i), end="")
            if i == self.lift:
                print(" E", end="")
            else:
                print("  ", end="")
            for j in self.floor[i]:
                print(" %s" % j, end="")
            print("")

    def make_move(self, m, indent):
        (to_floor, objs) = m
        for o in objs:
            # print("%smoving %s to floor %s" % (" " * indent, o, to_floor))
            self.floor[self.lift].remove(o)
            self.floor[to_floor].append(o)
        self.lift = to_floor
        # self.printstate(indent=indent)
        # self.printcompact(indent=indent)

    def solved(self):
        return len(self.floor[0]) == 0 and len(self.floor[1]) == 0 and \
               len(self.floor[2]) == 0

    def valid_floor(self, ary):
        c = list()
        g = list()
        chip_cnt = 0
        gen_cnt = 0
        for x in ary:
            if x.ischip:
                chip_cnt += 1
                c.append(x)
            else:
                gen_cnt += 1
                g.append(x)

        # if no generators or no chips, then we're ok
        if chip_cnt == 0 or gen_cnt == 0:
            return True

        # A generator can only be without its corresponding
        # chip only if every chip on the floor has a matching generator
        chips_unmatched = chip_cnt
        gens_unmatched = gen_cnt
        for chip in c:
            chipgen = chip.get_gen()
            if chipgen in g:
                chips_unmatched -= 1
                gens_unmatched -= 1

        assert(chips_unmatched >= 0)
        assert(gens_unmatched >= 0)

        if chips_unmatched == 0 or gens_unmatched == 0:
            return True

        return False


    def can_remove_from_floor(self, item):
        foo = list(self.floor[self.lift])
        foo.remove(item)
        result = self.valid_floor(foo)
        # if result:
        #     print("can remove %s from floor %s" % (item, self.lift))
        return result

    def can_add_to_floor(self, to, item):
        foo = list(self.floor[to]);
        foo.append(item)
        result = self.valid_floor(foo)
        # if result:
        #     print("can add %s to floor %s" % (item, to))
        return result
    
    def valid_move(self, to_floor, item):
        srcfloor = list(self.floor[self.lift])
        dstfloor = list(self.floor[to_floor]) 
        for i in item:
            srcfloor.remove(i)
            dstfloor.append(i)
        return self.valid_floor(srcfloor) and self.valid_floor(dstfloor)

    def find_moves(self):
        moves = list()
        possible = list()
        if self.lift < 3:
            possible.append(self.lift + 1)
        if self.lift > 0:
            possible.append(self.lift - 1)

        for f in possible:
            for i in itertools.chain.from_iterable(
                            itertools.combinations(self.floor[self.lift], r) 
                            for r in [1, 2]):
                # print(">>> %s" % list(i))
                if self.valid_move(f, i):
                    x = (f, i)
                    moves.append(x)

        return moves


def setup(cmds):
    rtf = Factory()
    fmap = { "first": 0, "second": 1, "third": 2, "fourth": 3 }
    f = re.compile(r"The (\S+) floor contains")
    chip = re.compile(r"[, ]*a[n]* (\S+)-compatible microchip")
    gen = re.compile(r"[, ]*a[n]* (\S+) generator")
    for c in cmds:
        m = f.match(c)
        if not m:
            print("unable to determine floor:\n\t%s" % c)
            sys.exit(1)
        floor = fmap[m.group(1)]
        x = len(m.group(0))
        what = c[x:]
        m = chip.match(what)
        if m:
            ischip = True
        else:
            m = gen.match(what)
            ischip = False
        while m:
            l = len(m.group(0))
            if ischip:
                rtf.place_chip(m.group(1), floor)
            else:
                rtf.place_gen(m.group(1), floor)
            what = what[l:]
            if what.startswith(" and "):
                what = what[4:]
            m = chip.match(what)
            if m:
                ischip = True
            else:
                m = gen.match(what)
                ischip = False

    return rtf


print("-------------------------------")

testdata = [
    "The first floor contains a hydrogen-compatible microchip " \
            "and a lithium-compatible microchip.",
    "The second floor contains a hydrogen generator.",
    "The third floor contains a lithium generator.",
    "The fourth floor contains nothing relevant.",
    ]

# factory = setup(testdata)
factory = setup([line.strip() for line in open("day11.txt")])

MINMOVES = 1000
STATES = list()

def permute(fact, mvcnt):
    global MINMOVES, STATES
    if mvcnt > 50:
        return None
    STATES.append(fact)
    moves = fact.find_moves()
    for m in moves:
        # print(" " * mvcnt, end="")
        # print("consider: %s" % m)
        nextfact = copy.deepcopy(fact)
        nextfact.make_move(m, mvcnt)
        if nextfact in STATES:
            continue
        if nextfact.solved():
            return mvcnt
        rslt = permute(nextfact, mvcnt + 1)
        if rslt is None:
            continue
        if rslt < MINMOVES:
            MINMOVES = rslt
            print("current min is %s" % MINMOVES)
    STATES.remove(fact)

factory.printstate()
permute(factory, 1)
print("final min is %s" % MINMOVES)
