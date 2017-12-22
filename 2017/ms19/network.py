class Diagram:
    HALT = 0
    UP = 1
    DOWN = 2
    LEFT = 3
    RIGHT = 4

    def __init__(self, grid=None):
        self.grid = grid
        self.seq = []

    def start(self):
        self.steps = 0
        return (self.grid[0].index("|"), 0, self.DOWN)

    def char_or_letter(self, x, y, ok):
        return str.isupper(self.grid[y][x]) or self.grid[y][x] == ok

    def check_left_right(self, x, y):
        if x - 1 >= 0 and self.char_or_letter(x - 1, y, '-'):
            return self.LEFT

        if x + 1 < len(self.grid[y]) and self.char_or_letter(x + 1, y, '-'):
            return self.RIGHT

    def check_up_down(self, x, y):
        if y - 1 >= 0 and self.char_or_letter(x, y - 1, '|'):
            return self.UP

        if y + 1 < len(self.grid) and self.char_or_letter(x, y + 1, '|'):
            return self.DOWN

    def moveFrom(self, pos):
        self.steps += 1
        (x, y, direc) = pos
        (cx, cy) = (x, y)
        if direc == self.DOWN:
            cy += 1
        elif direc == self.UP:
            cy -= 1
        elif direc == self.LEFT:
            cx -= 1
        elif direc == self.RIGHT:
            cx += 1
        else:
            raise Exception

        if cx < 0 or cx > len(self.grid[0]) or cy < 0 or cy > len(self.grid):
            return (-1, -1, self.HALT)
        if self.grid[cy][cx] == " ":
            return (-1, -1, self.HALT)

        if str.isupper(self.grid[cy][cx]):
            self.seq.append(self.grid[cy][cx])

        if self.grid[cy][cx] == "+":
            if direc == self.DOWN or direc == self.UP:
                newdir = self.check_left_right(cx, cy)
            else:
                newdir = self.check_up_down(cx, cy)
        else:
            newdir = direc

        return (cx, cy, newdir)


if __name__ == "__main__":
    puzzle = [line for line in open("puzzle_data.txt")]
    d = Diagram(puzzle)
    pos = d.start()
    while pos[2] != Diagram.HALT:
        pos = d.moveFrom(pos)
    print("Part 1: %s" % "".join(d.seq))
    print("Part 2: %s" % d.steps)
