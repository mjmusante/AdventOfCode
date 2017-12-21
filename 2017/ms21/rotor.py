import math
import re
import timeit


class Rotor:
    # a b c      c f i
    # d e f  ->  b e h
    # g h i      a d g
    ROTATE3 = [2, 5, 8, 1, 4, 7, 0, 3, 6]

    # a b c      c b a
    # d e f  ->  f e d
    # g h i      i h g
    FLIP3 = [2, 1, 0, 5, 4, 3, 8, 7, 6]

    # a b     b d
    # c d  -> a c
    ROTATE2 = [1, 3, 0, 2]

    # a b     b a
    # c d  -> d c
    FLIP2 = [1, 0, 3, 2]

    def __init__(self, istate=None):
        if istate is None:
            self.pos = ".#...####"
        else:
            self.pos = istate

    def state(self):
        return self.pos

    def rotate(self):
        s = ""
        if len(self.pos) == 9:
            r = self.ROTATE3
        else:
            r = self.ROTATE2

        for i in r:
            s += self.pos[i]

        return Rotor(s)

    def flip(self):
        s = ""
        if len(self.pos) == 9:
            r = self.FLIP3
        else:
            r = self.FLIP2
        for i in r:
            s += self.pos[i]
        return Rotor(s)

    def split(self):
        sz = int(math.sqrt(len(self.pos)))
        assert((sz * sz) == len(self.pos))

        if sz % 2 == 0:
            pattern = ".."
            multiple = 2
        else:
            pattern = "..."
            multiple = 3

        ans = []
        a = re.findall(pattern, self.pos)
        for row in range(sz / multiple):
            rdata = [""] * (sz / multiple)
            for col in range(sz):
                rdata[col % (sz / multiple)] += a.pop(0)
            ans.append([Rotor(r) for r in rdata])
        return ans

    def dump(self):
        if len(self.pos) == 9:
            print("%s" % "\n".join(re.findall("...", self.pos)))
        else:
            print("%s" % "\n".join(re.findall("..", self.pos)))

    def __repr__(self):
        return "'" + self.pos + "'"


class Pattern:

    def __init__(self):
        self.plist = dict()

    def add_pattern(self, newpat):
        (src, _, dst) = newpat.split(" ")
        src = "".join([x for x in src if x != "/"])
        dst = "".join([x for x in dst if x != "/"])

        r = Rotor(src)
        for i in range(4):
            if r.state() not in self.plist:
                self.plist[r.state()] = dst
                r = r.rotate()

        r = r.flip()
        for i in range(4):
            if r.state() not in self.plist:
                self.plist[r.state()] = dst
                r = r.rotate()

        r = r.rotate().flip()
        for i in range(4):
            if r.state() not in self.plist:
                self.plist[r.state()] = dst
                r = r.rotate()

    def apply_to(self, r):
        if r.state() not in self.plist:
            raise Exception
        return Rotor(self.plist[r.state()])


class Grid:

    def __init__(self, istate, patterns):
        self.grid = [[Rotor(istate)]]
        self.patterns = patterns

    def fracture(self):
        self.grid = self.grid[0][0].split()

    def rejoin(self, trace=False):
        if len(self.grid) == 1:
            return

        if len(self.grid[0][0].state()) == 4:
            # rejoining 2x2s
            pattern = ".."
            multiple = 2
        elif len(self.grid[0][0].state()) == 9:
            pattern = "..."
            multiple = 3
        else:
            assert(len(self.grid[0][0].state()) == 16)
            pattern = "...."
            multiple = 4

        newrows = [""] * (len(self.grid) * multiple)
        for row in range(len(self.grid)):
            for col in range(len(self.grid[row])):
                a = re.findall(pattern, self.grid[row][col].state())
                if trace:
                    print(a)
                for i in range(len(a)):
                    newrows[multiple * row + i] += a.pop(0)
        self.grid = [[Rotor("".join(newrows))]]

    def do_patterns(self):
        newgrid = []
        for row in self.grid:
            newrow = []
            for col in row:
                newrow.append(self.patterns.apply_to(col))
            newgrid.append(newrow)
        self.grid = newgrid

    def count_pixels(self):
        count = 0
        for row in self.grid:
            for col in row:
                count += col.state().count("#")

        return count

    def one_move(self):
        self.rejoin()
        self.fracture()
        self.do_patterns()


def cvt_secs(s):
    ms = int(math.floor(s * 1000)) % 1000

    rest = int(s)
    sec = rest % 60
    ans = "%s.%03ds" % (sec, ms)

    minutes = int(rest / 60)
    if minutes > 0:
        ans = "%sm" % minutes + ans

    return ans


if __name__ == "__main__":
    p = Pattern()
    lines = [line.strip() for line in open("puzzle_data.txt")]
    for l in lines:
        p.add_pattern(l)
    g = Grid(".#...####", p)

    for i in range(5):
        t = timeit.timeit(g.one_move, number=1)
        print("Iteration %s: %s" % (i, cvt_secs(t)))

    print("Part 1: %s" % g.count_pixels())

    for i in range(13):
        t = timeit.timeit(g.one_move, number=1)
        print("Iteration %s: %s" % (i + 5, cvt_secs(t)))

    print("Part 2: %s" % g.count_pixels())
