class FirstFloor:

    SPACE = 90
    WALL = 91

    def __init__(self, floor):
        self.designer = floor


    def get(self, x, y):
        base = x * x + 3 * x + 2 * x * y + y + y * y
        base += self.designer
        numones = bin(base).count("1")
        if numones % 2 == 0:
            return self.SPACE
        return self.WALL

    def dump(self, max_x, max_y):
        for y in range(max_y):
            str = "%3d " % y
            for x in range(max_x):
                if self.get(x, y) == self.SPACE:
                    str += "."
                else:
                    str += "#"
            print(str)

    def adjacent_spaces(self, midx, midy):
        for pos in [(midx - 1, midy), (midx, midy - 1),
                    (midx + 1, midy), (midx, midy + 1)]:
            if self.get(*pos) == self.SPACE:
                yield pos


    def find_shortest_path(self, start, end, path=[]):
        path = path + [start]
        if start == end:
            return path
        if start[0] > end[0] + 10 or start[1] > end[1] + 10:
            return None
        if len(path) > 150:
            return None
        if self.get(*start) == self.WALL:
            return None
        shortest = None
        for node in self.adjacent_spaces(*start):
            if node not in path:
                newpath = self.find_shortest_path(node, end, path)
                if newpath:
                    if not shortest or len(newpath) < len(shortest):
                        shortest = newpath
                        print(len(shortest))
        return shortest

if __name__ == "__main__":
    ff = FirstFloor(1350)
    print("steps to (31, 39) not including first = %s" %
          len(ff.find_shortest_path((1, 1), (31, 39))) - 1)