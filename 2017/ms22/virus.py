class Virus:

    UP = 1
    RIGHT = 2
    DOWN = 3
    LEFT = 4

    def __init__(self, memmap=None):
        if memmap:
            self.memmap = list(memmap)
        else:
            self.memmap = ["."]

        self.xsize = len(self.memmap[0])
        self.ysize = len(self.memmap)

        self.xoffset = -(self.xsize - 1) / 2
        self.yoffset = -(self.ysize - 1) / 2

        self.xpos = 0
        self.ypos = 0

        self.facing = self.UP
        self.infections = 0

    def turn_right(self):
        self.facing = self.facing % 4 + 1

    def turn_left(self):
        self.facing = (self.facing + 2) % 4 + 1

    def expand_upwards(self):
        self.memmap = ["." * len(self.memmap[0])] + self.memmap
        self.yoffset -= 1
        self.ysize += 1

    def expand_rightwards(self):
        for i in range(len(self.memmap)):
            self.memmap[i] += "." * 32
        self.xsize += 32

    def expand_leftwards(self):
        for i in range(len(self.memmap)):
            self.memmap[i] = "." * 32 + self.memmap[i]
        self.xoffset -= 32
        self.xsize += 32

    def expand_downwards(self):
        self.memmap.append("." * len(self.memmap[0]))
        self.ysize += 1

    def set_memloc(self, data):
        row = self.ypos - self.yoffset
        col = self.xpos - self.xoffset

        d = self.memmap[row]
        self.memmap[row] = d[:col] + data + d[col + 1:]

    def get_curloc_state(self):
        row = self.ypos - self.yoffset
        col = self.xpos - self.xoffset
        return self.memmap[row][col]

    def infected(self):
        return self.get_curloc_state() == '#'

    def weakened(self):
        return self.get_curloc_state() == 'w'

    def flagged(self):
        return self.get_curloc_state() == 'f'

    def next_infection_stage(self):
        s = self.get_curloc_state()
        if s == '.':
            self.set_memloc('w')
        elif s == 'w':
            self.set_memloc('#')
            self.infections += 1
        elif s == '#':
            self.set_memloc('f')
        elif s == 'f':
            self.set_memloc('.')
        else:
            raise Exception

    def loc(self):
        return (self.xpos, self.ypos)

    def direc(self):
        return self.facing

    def move_forward(self):
        if self.facing == self.UP:
            dx = 0
            dy = -1
        elif self.facing == self.RIGHT:
            dx = 1
            dy = 0
        elif self.facing == self.DOWN:
            dx = 0
            dy = 1
        elif self.facing == self.LEFT:
            dx = -1
            dy = 0
        else:
            raise Exception

        self.xpos += dx
        self.ypos += dy

        xp = self.xpos - self.xoffset
        yp = self.ypos - self.yoffset

        if xp < 0:
            self.expand_leftwards()
        elif xp >= self.xsize:
            self.expand_rightwards()

        if yp < 0:
            self.expand_upwards()
        elif yp >= self.ysize:
            self.expand_downwards()

        xp = self.xpos - self.xoffset
        yp = self.ypos - self.yoffset

        assert(xp >= 0 and xp < self.xsize)
        assert(yp >= 0 and yp < self.ysize)

    def infect(self):
        assert(not self.infected())
        self.set_memloc("#")

    def clean(self):
        assert(self.infected())
        self.set_memloc(".")

    def burst(self):
        # 1. if the current node is infected, turn right. otherwise turn left
        if self.infected():
            self.turn_right()
        else:
            self.turn_left()

        # 2. if node is clean, infected it. otherwise clean it
        if self.infected():
            self.clean()
        else:
            self.infect()
            self.infections += 1

        # 3. move forward
        self.move_forward()

    def evolved_burst(self):
        # 1. decide which way to go
        s = self.get_curloc_state()
        if s == '#':
            self.turn_right()
        elif s == 'f':
            self.turn_right()
            self.turn_right()
        elif s != 'w':
            self.turn_left()

        # 2. change current location to next stage
        self.next_infection_stage()

        # 3. move forward
        self.move_forward()


if __name__ == "__main__":
    puzzle = [line.strip() for line in open("puzzle_data.txt")]

    part1 = Virus(puzzle)
    for i in range(10000):
        part1.burst()
    print("Part 1: %s" % part1.infections)

    part2 = Virus(puzzle)
    for i in range(10000000):
        part2.evolved_burst()
    print("Part 2: %s" % part2.infections)
