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
        self.xoffset = -(len(self.memmap[0]) - 1) / 2
        self.yoffset = -(len(self.memmap) - 1) / 2
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

    def expand_rightwards(self):
        for i in range(len(self.memmap)):
            self.memmap[i] += "."

    def expand_leftwards(self):
        for i in range(len(self.memmap)):
            self.memmap[i] = "." + self.memmap[i]
        self.xoffset -= 1

    def expand_downwards(self):
        self.memmap.append("." * len(self.memmap[0]))

    def set_memloc(self, data):
        row = self.ypos - self.yoffset
        col = self.xpos - self.xoffset

        d = self.memmap[row]
        self.memmap[row] = d[:col] + data + d[col + 1:]

    def infected(self):
        row = self.ypos - self.yoffset
        col = self.xpos - self.xoffset

        return self.memmap[row][col] == '#'

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
        elif xp >= len(self.memmap[0]):
            self.expand_rightwards()

        if yp < 0:
            self.expand_upwards()
        elif yp >= len(self.memmap):
            self.expand_downwards()

        xp = self.xpos - self.xoffset
        yp = self.ypos - self.yoffset

        assert(xp >= 0 and xp < len(self.memmap[0]))
        assert(yp >= 0 and yp < len(self.memmap))

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
