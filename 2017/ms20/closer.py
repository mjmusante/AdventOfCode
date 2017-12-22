import math
import re


class Particle:

    def __init__(self, pos, vel, acc):
        self.pos = pos
        self.vel = vel
        self.acc = acc

    def collides_with(self, other):
        return self.pos == other.pos


class Closer:
    AWAY = 1
    TOWARDS = 2

    SLOWING_DOWN = 3
    SPEEDING_UP = 4

    def __init__(self, lines=None):

        self.particles = []

        if not lines:
            return

        for l in lines:
            num = re.findall("-?\d+", l)
            pos = [int(i) for i in num[:3]]
            vel = [int(i) for i in num[3:6]]
            acc = [int(i) for i in num[6:]]
            self.particles.append(Particle(pos, vel, acc))

    def step(self):
        for p in self.particles:
            p.vel = [p.vel[i] + p.acc[i] for i in range(3)]
            p.pos = [p.pos[i] + p.vel[i] for i in range(3)]

    def distance(self, pos):
        return sum([abs(x) for x in pos])

    def closest(self):
        min_d = None
        for i in range(len(self.particles)):
            d = self.distance(self.particles[i].pos)
            if not min_d or min_d > d:
                min_d = d
                ans = i
        return ans

    def delta_p(self):
        old_p = [x.pos for x in self.particles]
        self.step()
        new_p = [x.pos for x in self.particles]

        return [self.distance(new_p[i]) - self.distance(old_p[i])
                for i in range(len(self.particles))]

    def delta_v(self):
        ans = []
        for p in self.particles:
            old_v = self.distance(p.vel)
            new_v = self.distance([p.vel[i] + p.acc[i] for i in range(3)])
            if new_v - old_v > 0:
                ans.append(self.SPEEDING_UP)
            else:
                ans.append(self.SLOWING_DOWN)
        return ans

    def direction(self):
        delta = self.delta_p()

        ans = []
        for d in delta:
            if d < 0:
                ans.append(self.TOWARDS)
            else:
                ans.append(self.AWAY)
        return ans

    def smallest_acc(self):
        smallest = None
        for i in range(len(self.particles)):
            acc = self.distance(self.particles[i].acc)
            if smallest is None or smallest > acc:
                smallest = acc
                which = i
        return which

    def remove_collisions(self):
        foo = dict()
        for p in self.particles:
            s = str(p.pos)
            if s in foo:
                foo[s] = 0
            else:
                foo[s] = p
        self.particles = [foo[p] for p in foo if foo[p] != 0]


if __name__ == "__main__":
    plist = [line.strip() for line in open("puzzle_data.txt")]
    c = Closer(plist)
    print("Part 1: %s" % c.smallest_acc())

    # This is a hack: we know the answer is less than 50.
    # To fix this the right way, the answer can be determined
    # by solving a quadratic equation to determine whther any
    # two particles occupy the same location at a given time t
    for i in range(50):
        c.step()
        c.remove_collisions()

    print("Part 2: %s" % len(c.particles))
