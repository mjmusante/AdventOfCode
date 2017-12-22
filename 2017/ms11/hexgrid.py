def pathdist(ary):
    x = 0
    y = 0
    mdist = 0
    curdist = 0
    for p in ary.split(","):
        if p == "ne":
            x += 1
        elif p == "sw":
            x -= 1
        elif p == "se":
            y -= 1
            x += 1
        elif p == "nw":
            y += 1
            x -= 1
        elif p == "n":
            y += 1
        elif p == "s":
            y -= 1
        mdist = max(mdist, max(abs(x), abs(y), abs(x + y)))
    return (max(abs(x), abs(y), abs(x + y)), mdist)


disttest = {
    "ne,ne,ne": 3,
    "ne,ne,sw,sw": 0,
    "ne,ne,s,s": 2,
    "se,sw,se,sw,sw": 3
}

if __name__ == "__main__":
    for d in disttest:
        pd = pathdist(d)
        if pd[0] != disttest[d]:
            print("for '%s': expecting %s got %s" % (d, disttest[d], pd[0]))

    steps = [line.strip() for line in open("puzzle_data.txt")]
    (dist, mdist) = pathdist(steps[0])
    print("Part 1: %s" % dist)
    print("Part 2: %s" % mdist)
