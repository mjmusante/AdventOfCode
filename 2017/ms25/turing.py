
class State:

    def __init__(self, name, w0, m0, n0, w1, m1, n1):
        self.name = name
        self.w0 = w0
        self.m0 = m0
        self.n0 = n0
        self.w1 = w1
        self.m1 = m1
        self.n1 = n1

    def write(self, val):
        if val == 0:
            return self.w0
        return self.w1

    def move(self, val):
        if val == 0:
            return self.m0
        return self.m1

    def nstate(self, val):
        if val == 0:
            return self.n0
        return self.n1


class Turing:

    def __init__(self):
        self.mach = dict()
        self.mach['a'] = State('a', 1,  1, 'b', 0, -1, 'e')
        self.mach['b'] = State('b', 1, -1, 'c', 0,  1, 'a')
        self.mach['c'] = State('c', 1, -1, 'd', 0,  1, 'c')
        self.mach['d'] = State('d', 1, -1, 'e', 0, -1, 'f')
        self.mach['e'] = State('e', 1, -1, 'a', 1, -1, 'c')
        self.mach['f'] = State('f', 1, -1, 'e', 1,  1, 'a')
        # self.mach['a'] = State('a', 1,  1, 'b', 0, -1, 'b')
        # self.mach['b'] = State('b', 1, -1, 'a', 1,  1, 'a')
        self.tape = [0] * 1024
        self.pos = 512
        self.state = 'a'

    def step(self):
        m = self.mach[self.state]
        p = self.pos
        v = self.tape[p]

        # print(" -> s=%s, w=%s, p=%s" % (m.nstate(v), m.write(v), self.pos + m.move(v)))
        self.state = m.nstate(v)
        self.tape[p] = m.write(v)
        self.pos += m.move(v)

        if self.pos < 0:
            self.tape = [0] * 1024 + self.tape
            self.pos += 1024
        if self.pos >= len(self.tape):
            self.tape += [0] * 1024


if __name__ == "__main__":
    t = Turing()
    for i in range(12386363):
        # print("%s %s : sum=%s" % (t.state, t.tape[t.pos], sum(t.tape)))
        # if raw_input() == 'q':
        #     break
        t.step()

    print("Part 1: %s" % sum(t.tape))
