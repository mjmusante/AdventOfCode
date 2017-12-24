import operator


class Bridge:

    def __init__(self, ports=None, movelist=None, bridge=[]):
        self.ports = ports
        if ports is None:
            if movelist is not None:
                self.ports = list(movelist)
        else:
            self.ports = [[int(x[0]), int(x[1])] for x in
                          [x.split("/") for x in ports]]
        self.bridge = bridge

    def find_moves(self):
        if self.bridge == []:
            x = 0
        else:
            x = self.bridge[-1]

        ans = []
        for p in self.ports:
            if x in p:
                ans.append(list(p))
        return ans

    def strength(self):
        return reduce(operator.add, self.bridge, 0)

    def attach(self, move):
        movelist = [p for p in self.ports if p != move]
        stage = Bridge(movelist=movelist, bridge=self.bridge)

        if stage.bridge == []:
            stage.bridge = move
        else:
            stage.bridge = list(self.bridge)
            x = self.bridge[-1]
            if x == move[1]:
                move.reverse()
            assert(x == move[0])
            stage.bridge += move
        return stage

    def move_strength(self, move):
        return move[0] + move[1]

    def solve(self):
        mlist = self.find_moves()
        best = None
        for m in mlist:
            nb = self.attach(m)
            ns = nb.solve()
            if ns is not None:
                if best is None or best < ns:
                    best = ns

        if best is None:
            best = self.strength()

        return best

    def solve_part_2(self):
        mlist = self.find_moves()
        best = (None, None)     # (length, strength)
        for m in mlist:
            nb = self.attach(m)
            (nl, ns) = nb.solve_part_2()
            if nl is not None:
                if best == (None, None):
                    best = (nl, ns)
                elif best[0] < nl or (best[0] == nl and best[1] < ns):
                    best = (nl, ns)
        if best == (None, None):
            best = (len(self.bridge) / 2, self.strength())

        return best


if __name__ == "__main__":
    puzzle = [line.strip() for line in open("puzzle_data.txt")]
    b = Bridge(ports=puzzle)
    print("Part 1: %s" % b.solve())
    print("Part 2: %s" % b.solve_part_2()[1])
