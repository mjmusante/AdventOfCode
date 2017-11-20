import re

class AssemBunny:
    INCREG = re.compile("inc ([a-d])")
    DECREG = re.compile("dec ([a-d])")
    CPYREG = re.compile("cpy ([a-d]) ([a-d])")
    CPYNUM = re.compile("cpy (-?\d*) ([a-d])")
    JNZREG = re.compile("jnz ([a-d]) (-?\d*)")
    JNZNUM = re.compile("jnz (-?\d*) (-?\d*)")


    def __init__(self, immcode=None, pathname=None):
        self.reg = {"a": 0, "b": 0, "c": 0, "d": 0}
        self.ip = 0
        if immcode:
            self.inst = [line.strip() for line in immcode.split(";")]
        elif pathname:
            self.inst = [line.strip() for line in open(pathname)]
        else:
            raise Exception("No instructions")

    def inc_reg(self, m):
        self.reg[m.group(1)] += 1

    def cpy_num(self, m):
        self.reg[m.group(2)] = int(m.group(1))

    def cpy_reg(self, m):
        self.reg[m.group(2)] = self.reg[m.group(1)]

    def dec_reg(self, m):
        self.reg[m.group(1)] -= 1

    def jnz_reg(self, m):
        if 0 != self.reg[m.group(1)]:
            self.ip += int(m.group(2)) - 1

    def jnz_num(self, m):
        if 0 != int(m.group(1)):
            self.ip += int(m.group(2)) - 1

    CMDS = {INCREG: inc_reg, CPYNUM: cpy_num, CPYREG: cpy_reg,
        DECREG: dec_reg, JNZREG: jnz_reg, JNZNUM: jnz_num}


    def run(self):
        while self.ip < len(self.inst):
            x = False
            for c, f in self.CMDS.iteritems():
                m = c.match(self.inst[self.ip])
                if m:
                    f(self, m)
                    x = True
                    break
            if not x:
                raise Exception("Illegal instruction: %s" % self.inst[self.ip])
            self.ip += 1

if __name__ == "__main__":
    assem = AssemBunny(pathname="day12.txt")
    assem.run()
    print("1st run - register a = %s" % assem.reg['a'])
    assem = AssemBunny(pathname="day12.txt")
    assem.reg['c'] = 1
    assem.run()
    print("2nd run - register a = %s" % assem.reg['a'])