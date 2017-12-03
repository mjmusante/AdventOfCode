import math

class Spiral:
    def __init__(self):
        self.grid = dict()
        self.grid[(0, 0)] = 1

    def coords_of(self, pos):
        k = math.ceil((math.sqrt(pos) - 1) / 2)
        t = 2 * k + 1
        m = t * t
        t -= 1
        if pos >= m - t:
            return (k - (m - pos), -k)
        m = m - t
        if pos >= m - t:
            return (-k, -k + (m - pos))
        m = m - t
        if pos >= m - t:
            return (-k + (m - pos), k)
        return (k, k - (m - pos - t))

    def steps_from_1(self, addr):
        datap = 1
        xnext = 0
        ynext = 0

        xcur = 0
        ycur = 0

        stage = 0
        while datap < addr:
            # print("(%s, %s) -> %s" % (xcur, ycur, datap))
            if stage == 0:
                xnext += 1
                datap += (xnext - xcur)
                xcur = xnext
                stage = 1
            elif stage == 1:
                ynext += 1
                datap += (ynext - ycur)
                ycur = ynext
                stage = 2
            elif stage == 2:
                datap += 2 * xcur
                xcur = -xcur
                stage = 3
            elif stage == 3:
                datap += 2 * ycur
                ycur = -ycur
                stage = 0
            else:
                assert(False)
        # print("(%s, %s) -> %s" % (xcur, ycur, datap))
        return abs(xcur) + abs(ycur) - (datap - addr)

    def sum_larger_than(self, val):
        datap = 1
        while True:
            x, y = self.coords_of(datap)
            if (x, y) in self.grid:
                if self.grid[(x, y)] > val:
                    return self.grid[(x, y)]
                datap += 1
                continue
            else:
                cursum = 0
                for xp in [x - 1, x, x + 1]:
                    for yp in [y - 1, y, y + 1]:
                        if (xp, yp) in self.grid:
                            cursum += self.grid[(xp, yp)]
                self.grid[(x, y)] = cursum
                if cursum > val:
                    return cursum
            datap += 1

if __name__ == "__main__":
    s = Spiral()
    print("Part 1: %s" % s.steps_from_1(277678))
    print("Part 2: %s" % s.sum_larger_than(277678))