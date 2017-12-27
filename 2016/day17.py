import md5


class Vault:

    def __init__(self, passcode, extend=None, direc="", pos=(0, 0), mc=0):
        if extend:
            self.passcode = passcode.copy()
            self.passcode.update(extend)
            self.direc = direc + extend
        else:
            self.passcode = md5.new(passcode)
            self.direc = direc
        self.pos = pos
        self.mc = mc
        self.seq = []

    def loc(self):
        return self.pos

    def get_moves(self):
        rslt = []
        mv = list(self.passcode.hexdigest()[:4])
        if mv[0] >= 'b' and self.pos[1] > 0:
            rslt.append('u')
        if mv[1] >= 'b' and self.pos[1] < 3:
            rslt.append('d')
        if mv[2] >= 'b' and self.pos[0] > 0:
            rslt.append('l')
        if mv[3] >= 'b' and self.pos[0] < 3:
            rslt.append('r')
        return rslt

    def move_up(self):
        return Vault(self.passcode, extend='U', direc=self.direc,
                     pos=(self.pos[0], self.pos[1] - 1), mc=self.mc + 1)

    def move_down(self):
        return Vault(self.passcode, extend='D', direc=self.direc,
                     pos=(self.pos[0], self.pos[1] + 1), mc=self.mc + 1)

    def move_left(self):
        return Vault(self.passcode, extend='L', direc=self.direc,
                     pos=(self.pos[0] - 1, self.pos[1]), mc=self.mc + 1)

    def move_right(self):
        return Vault(self.passcode, extend='R', direc=self.direc,
                     pos=(self.pos[0] + 1, self.pos[1]), mc=self.mc + 1)

    def find_shortest_path(self):
        assert(self.pos != (3, 3))

        if self.mc > 50:
            return None

        moves = self.get_moves()
        if not moves:
            return None

        best = None
        for m in moves:
            if m == 'u':
                v = self.move_up()
            elif m == 'd':
                v = self.move_down()
            elif m == 'l':
                v = self.move_left()
            elif m == 'r':
                v = self.move_right()
            else:
                raise Exception

            if v.pos == (3, 3):
                if best is None or best.mc > v.mc:
                    best = v
                    self.seq = [m] + v.seq
            else:
                attempt = v.find_shortest_path()
                if attempt is not None:
                    if best is None or best.mc > attempt.mc:
                        best = attempt
                        self.seq = [m] + v.seq

        return best

    def find_longest_path(self):
        assert(self.pos != (3, 3))

        if self.mc > 950:
            return None

        moves = self.get_moves()
        if not moves:
            return None

        best = None
        for m in moves:
            if m == 'u':
                v = self.move_up()
            elif m == 'd':
                v = self.move_down()
            elif m == 'l':
                v = self.move_left()
            elif m == 'r':
                v = self.move_right()
            else:
                raise Exception

            if v.pos == (3, 3):
                if best is None or best.mc < v.mc:
                    best = v
                    self.seq = [m] + v.seq
            else:
                attempt = v.find_longest_path()
                if attempt is not None:
                    if best is None or best.mc < attempt.mc:
                        best = attempt
                        self.seq = [m] + v.seq

        return best


if __name__ == "__main__":
    v = Vault("hhhxzeay")
    print("Part 1: %s" % v.find_shortest_path().direc)
    print("Part 2: %s" % v.find_longest_path().mc)
