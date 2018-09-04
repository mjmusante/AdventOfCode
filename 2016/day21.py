import itertools
import re
import sys


class Hasher:

    def __init__(self, str=""):
        self.mystring = list(str)
        re_list = [
            (r"swap position (\d) with position (\d)", self.swap_pos),
            (r"swap letter (\w) with letter (\w)", self.swap_letter),
            (r"rotate (\w*) (\d) step[s]*", self.rotate),
            (r"rotate based on position of letter (\w)", self.rotate_pos),
            (r"reverse positions (\d) through (\d)", self.reverse),
            (r"move position (\d) to position (\d)", self.move),
        ]
        self.cmd = list()
        for r in re_list:
            self.cmd.append((re.compile(r[0]), r[1]))

    def get_hash(self):
        return "".join(self.mystring)

    def do_nothing(self, m):
        pass

    def do_swap(self, i1, i2):
        t = self.mystring[i1]
        self.mystring[i1] = self.mystring[i2]
        self.mystring[i2] = t

    def do_rotate(self, r):
        self.mystring = self.mystring[-r:] + self.mystring[:-r]

    def swap_pos(self, m):
        x1 = int(m.group(1))
        x2 = int(m.group(2))
        self.do_swap(x1, x2)

    def swap_letter(self, m):
        l1 = m.group(1)
        l2 = m.group(2)
        i1 = self.mystring.index(l1)
        i2 = self.mystring.index(l2)
        self.do_swap(i1, i2)

    def rotate(self, m):
        rval = int(m.group(2)) % len(self.mystring)
        if m.group(1) == "left":
            self.do_rotate(-rval)
        elif m.group(1) == "right":
            self.do_rotate(rval)
        else:
            print("fatal rotation")
            sys.exit(1)

    def rotate_pos(self, m):
        l1 = m.group(1)
        i1 = self.mystring.index(l1)
        if i1 >= 4:
            i1 += 1
        self.do_rotate((i1 + 1) % len(self.mystring))

    def reverse(self, m):
        p1 = int(m.group(1))
        p2 = int(m.group(2))
        if p1 == 0:
            midstr = self.mystring[p2::-1]
        else:
            midstr = self.mystring[p2:p1 - 1:-1]

        self.mystring = \
            self.mystring[:p1] + midstr + self.mystring[p2 + 1:]

    def move(self, m):
        p1 = int(m.group(1))
        p2 = int(m.group(2))
        t = self.mystring[p1]
        self.mystring = self.mystring[:p1] + self.mystring[p1 + 1:]
        self.mystring[p2:p2] = [t]

    def execute(self, cmds):
        # print(">> START %s" % "".join(self.mystring))
        for c in cmds:
            ok = False
            for x in self.cmd:
                m = x[0].match(c)
                if m:
                    # print("executing '%s'" % c)
                    x[1](m)
                    ok = True
                    # print(">> Next: %s" % "".join(self.mystring))
                    break
            if not ok:
                print("fatal on '%s'" % c)
                sys.exit(1)


if __name__ == "__main__":
    lines = [l.strip() for l in open("day21.txt")]
    x = Hasher("abcdefgh")
    x.execute(lines)
    print("Part 1: %s" % x.get_hash())

    for i in itertools.permutations("abcdefgh"):
        x.mystring = list(i)
        x.execute(lines)
        if x.get_hash() == "fbgdceah":
            print("Part 2: %s" % "".join(i))
            break
