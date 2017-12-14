import sys
sys.path.append("..")

from ms10.knot import calc_hash

def hex_to_bin(hexstr):
    ans = ""
    for c in hexstr:
        b = bin(int(c, 16))[2:]
        ans += "0000"[len(b):] + b
    assert(len(ans) == 128)
    return ans


def is_group(grid, x, y):
    if grid[x][y] != '1':
        return 0

    def visit_group(x, y):
        if x < 0 or x > 127 or y < 0 or y > 127:
            return
        if grid[x][y] != '1':
            return
        grid[x][y] = '2'
        visit_group(x - 1, y)
        visit_group(x, y - 1)
        visit_group(x + 1, y)
        visit_group(x, y + 1)

    visit_group(x, y)
    return 1


if __name__ == "__main__":
    count = 0
    grid = []
    for i in range(128):
        b = hex_to_bin(calc_hash("jzgqcdpd-%s" % i))
        grid.append(list(b))
        count += b.count('1')
    print("Part 1: %s" % count)

    groups = 0
    for x in range(128):
        for y in range(128):
            groups += is_group(grid, x, y)
    print("Part 2: %s" % groups)
