import re
import sys


class IllegalInstructionException(Exception):

    def __init__(self, message, cmd):
        self.cmd = cmd
        super(IllegalInstructionException, self).__init__(message)


class Chip:
    REGLIST = "abcdefghijklmnopqrstuvwxyz"
    SET = re.compile(r'set ([a-z]) ([a-z]|-?\d+)$')
    SND = re.compile(r'snd ([a-z]|-?\d+)$')
    RCV = re.compile(r'rcv ([a-z])$')
    ADD = re.compile(r'add ([a-z]) ([a-z]|-?\d+)$')
    MUL = re.compile(r'mul ([a-z]) ([a-z]|-?\d+)$')
    MOD = re.compile(r'mod ([a-z]) ([a-z]|-?\d+)$')
    JGZ = re.compile(r'jgz ([a-z]|-?\d+) ([a-z]|-?\d+)$')

    def __init__(self, chip_id=0):
        self.reg = {}
        for r in self.REGLIST:
            self.reg[r] = 0
        self.snd = 0
        self.pc = 0
        self.waiting_reg = None
        self.queue = []
        self.sent_count = 0
        self.reg['p'] = chip_id

    def get_val(self, thing):
        try:
            return int(thing)
        except ValueError:
            return self.reg[thing]

    def get_reg(self, r, cmd):
        if r not in self.reg:
            print("'%s' is not a valid register in '%s'" % (r, cmd))
            sys.exit(1)
        return r

    def ex(self, cmd):
        self.pc += 1

        m = self.SET.match(cmd)
        if m:
            r = self.get_reg(m.group(1), cmd)
            s = self.get_val(m.group(2))
            self.reg[r] = s
            return (self.SET, True)

        m = self.SND.match(cmd)
        if m:
            self.snd = self.get_val(m.group(1))
            self.queue.append(self.snd)
            self.sent_count += 1
            return (self.SND, True)

        m = self.RCV.match(cmd)
        if m:
            r = self.get_reg(m.group(1), cmd)
            self.waiting_reg = r
            if self.reg[r] != 0:
                self.reg[r] = self.snd
                return (self.RCV, True)
            return (self.RCV, False)

        m = self.ADD.match(cmd)
        if m:
            r = self.get_reg(m.group(1), cmd)
            a = self.get_val(m.group(2))
            self.reg[r] += a
            return (self.ADD, True)

        m = self.MUL.match(cmd)
        if m:
            r = self.get_reg(m.group(1), cmd)
            p = self.get_val(m.group(2))
            self.reg[r] *= p
            return (self.MUL, True)

        m = self.MOD.match(cmd)
        if m:
            r = self.get_reg(m.group(1), cmd)
            p = self.get_val(m.group(2))
            self.reg[r] %= p
            return (self.MOD, True)

        m = self.JGZ.match(cmd)
        if m:
            r = self.get_val(m.group(1))
            d = self.get_val(m.group(2))
            if r > 0:
                self.pc += d - 1
                return (self.JGZ, True)
            return (self.JGZ, False)

        raise IllegalInstructionException(cmd, "cannot parse")

    def run_loopback(self, prg):
        while self.pc < len(prg):
            (inst, success) = self.ex(prg[self.pc])
            if success and inst == Chip.RCV:
                return

    def run(self, prg):
        while self.pc < len(prg):
            (inst, success) = self.ex(prg[self.pc])
            if inst == Chip.RCV:
                return

    def cont(self, prg, rcv_val):
        self.reg[self.waiting_reg] = rcv_val
        self.waiting_reg = None
        self.run(prg)

    def qpop(self):
        ans = self.queue[0]
        self.queue = self.queue[1:]
        return ans

    def has_values(self):
        return len(self.queue) > 0

    def is_blocked(self):
        return self.waiting_reg != None


def dual_run_prg(prg):
    c1 = Chip(0)
    c2 = Chip(1)

    c1.run(prg)
    c2.run(prg)

    # at this point, they're both blocked on rcv
    while True:
        while c1.has_values() and c2.is_blocked():
            val = c1.qpop()
            c2.cont(prg, val)

        while c2.has_values() and c1.is_blocked():
            val = c2.qpop()
            c1.cont(prg, val)

        if (c1.is_blocked() and not c2.has_values()) and \
                (c2.is_blocked() and not c1.has_values()):
            break

        if not c1.is_blocked() and not c2.is_blocked():
            break

    return (c1, c2)


if __name__ == "__main__":
    prg = [line.strip() for line in open("puzzle_data.txt")]
    c = Chip()
    c.run_loopback(prg)
    print("Part 1: %s" % c.snd)

    (p0, p1) = dual_run_prg(prg)
    print("Part 2: %s" % p1.sent_count)
