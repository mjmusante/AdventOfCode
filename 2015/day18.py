#! /usr/bin/python

import sys

with open("day18.txt") as f:
    lines = f.read().splitlines()

def turn_on_corners(grid):
    g0 = list(grid[0])
    g99 = list(grid[99])
    g0[0] = '#'
    g0[99] = '#'
    g99[0] = '#'
    g99[99] = '#'
    grid[0] = "".join(g0)
    grid[99] = "".join(g99)

def num_on(grid):
    oncount = 0
    for i in range(0, len(grid)):
        for j in range(0, len(grid[i])):
            if grid[i][j] == '#':
                oncount += 1
    return oncount

def one_step(grid):
    result = []
    for i in range(0, len(grid)):
        cur_row = []
        for j in range(0, len(grid[i])):
            oncount = 0
            for x in [i - 1, i, i + 1]:
                for y in [j - 1, j, j + 1]:
                    if x < 0 or x >= len(grid[i]) \
                            or y < 0 or y >= len(grid) \
                            or (x == i and y == j):
                        continue
                    if grid[x][y] == '#':
                        oncount += 1

            if grid[i][j] == '#':
                if oncount < 2 or oncount > 3:
                    cur_row.append(".")
                else:
                    cur_row.append("#")
            elif oncount == 3:
                cur_row.append("#")
            else:
                cur_row.append(".")
        if len(cur_row) != len(grid[i]):
            print("%s vs %s???" % (len(cur_row), len(grid[i])))
            sys.exit(1)
        result.append(cur_row)
    if len(result) != len(grid):
        print("%s vs %s??!?" % (len(result), len(grid)))
    return result

print("Number of lights on: %s" % num_on(lines))
grid = one_step(lines)
print("Number of ligths on after one step: %s" % num_on(grid))
for i in range(0, 99):
    grid = one_step(grid)
print("Number of ligths on after a hundred steps: %s" % num_on(grid))

print("Turning on corners.")
turn_on_corners(lines)
print("Number of lights on now: %s" % num_on(lines))
grid = one_step(lines)
turn_on_corners(grid)
print("If the corners are stuck on, after one step: %s" % num_on(grid))
for i in range(0, 99):
    grid = one_step(grid)
    turn_on_corners(grid)
print("and now, after 100 steps: %s" % num_on(grid))
