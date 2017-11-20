class FirstFloor:

    SPACE = 90
    WALL = 91

    def __init__(self, floor):
        self.designer = floor
        self.entries = dict()
        self.visited = dict()
        self.lowest = 100


    def get_old_way(self, x, y):
        base = x * x + 3 * x + 2 * x * y + y + y * y
        base += self.designer
        numones = bin(base).count("1")
        if numones % 2 == 0:
            return self.SPACE
        return self.WALL

    def get(self, x, y):
        if (x, y) not in self.entries:
            base = x * x + 3 * x + 2 * x * y + y + y * y
            base += self.designer
            numones = bin(base).count("1")
            if numones % 2 == 0:
                self.entries[(x, y)] = self.SPACE
            else:
                self.entries[(x, y)] = self.WALL
        return self.entries[(x, y)]

    def dump(self, max_x, max_y):
        for y in range(max_y):
            str = "%3d " % y
            for x in range(max_x):
                if (x, y) in self.visited:
                    if self.get(x, y) != self.SPACE:
                        print("ERROR on (%s, %s)" % (x, y))
                    else:
                        str += "o"
                else:
                    if self.get(x, y) == self.SPACE:
                        str += "."
                    else:
                        str += "#"
            print(str)

    def adjacent_spaces(self, midx, midy):
        for pos in [(midx - 1, midy), (midx, midy - 1),
                    (midx + 1, midy), (midx, midy + 1)]:
            if pos[0] < 0 or pos[1] < 0:
                continue
            if self.get(*pos) == self.SPACE:
                yield pos


    def visit_max_locs(self, start, path=[], lim=50):
        path = path + [start]
        if len(path) > lim + 1:
            return None
        if self.get(*start) == self.WALL:
            return None
        self.visited[start] = True
        # print(path)
        for node in self.adjacent_spaces(*start):
            if node not in path:
                self.visit_max_locs(node, path, lim)
        return len(self.visited)

    def find_shortest_path(self, start, end, path=[]):
        path = path + [start]
        if start == end:
            return path
        if start[0] > end[0] + 10 or start[1] > end[1] + 10:
            return None
        if len(path) > self.lowest:
            return None
        if self.get(*start) == self.WALL:
            return None
        shortest = None
        for node in self.adjacent_spaces(*start):
            if node not in path:
                newpath = self.find_shortest_path(node, end, path)
                if newpath:
                    if shortest:
                        l = len(shortest)
                    else:
                        l = len(newpath)
                    if not shortest or len(newpath) < l:
                        shortest = newpath
                        if l < self.lowest:
                            self.lowest = l
        return shortest

if __name__ == "__main__":
    ff = FirstFloor(1350)
    steps = len(ff.find_shortest_path((1, 1), (31, 39)))
    print("steps to (31, 39) not including first = %s" % (steps - 1))
    print("max places at 50 limit: %s" % (ff.visit_max_locs((1, 1))))
