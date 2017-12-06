import math

#
# This function returns the x, y coordinates
# of a "spiral" layout on a cartesian plane,
# with the first location at position (0, 0)
#
#   17 16 15 14 13
#   18  5  4  3 12
#   19  6  1  2 11
#   20  7  8  9 10
#
def coords_of(pos):
    k = math.ceil((math.sqrt(pos) - 1) / 2)
    t = 2 * k + 1
    m = t * t
    t -= 1
    if pos >= m - t:
        return (k - (m - pos), -k)
    m -= t
    if pos >= m - t:
        return (-k, -k + (m - pos))
    m -= t
    if pos >= m - t:
        return (-k + (m - pos), k)
    return (k, k - (m - pos - t))


#
# This function steps through the spiral memory and
# sets each new location to the sum of the 8 values
# surrounding it. It returns the first sum that's
# larger than the input value
def sum_larger_than(val):
    datap = 1
    grid = dict();
    grid[(0, 0)] = 1
    while True:
        x, y = coords_of(datap)
        cursum = 0
        for xp in [x - 1, x, x + 1]:
            for yp in [y - 1, y, y + 1]:
                if (xp, yp) in grid:
                    cursum += grid[(xp, yp)]
        grid[(x, y)] = cursum
        if cursum > val:
            return cursum
        datap += 1

if __name__ == "__main__":
    x, y = coords_of(277678)
    print("Part 1: %s" % int(abs(x) + abs(y)))
    print("Part 2: %s" % sum_larger_than(277678))
