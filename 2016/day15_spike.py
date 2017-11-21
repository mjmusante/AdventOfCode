

class Disc:
    def __init__(self, pos, phase):
        self.pos = pos
        self.phase = phase

    def pass_thru_at_time(self, t):
        return (self.phase + t) % self.pos == 0


if __name__ == "__main__":
    # d1 = Disc(5, 4)
    # d2 = Disc(2, 1)

    # dlist = [d1, d2]
    dlist = [Disc(17, 1), Disc(7, 0), Disc(19, 2), Disc(5, 0),
             Disc(3, 0), Disc(13, 5), Disc(11, 0)]

    t = 0
    while True:
        now = t + 1
        success = True
        for d in dlist:
            success = success and d.pass_thru_at_time(now)
            if not success:
                break
            now += 1
        if success:
            print("success at t=%s" % t)
            break
        t += 1
