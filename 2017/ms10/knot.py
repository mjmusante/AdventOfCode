
def crypt(ary, pos=0, skip=0, size=256, data=None):
    if not data:
        data = range(size)
    for s in ary:
        front = data[:s]
        back = data[s:]
        front.reverse()
        newdata = back + front
        while skip > len(data):
            skip -= len(data)
        data = newdata[skip:] + newdata[:skip]
        pos -= s + skip
        while pos < 0:
            pos += size
        skip += 1
    return (pos, skip, data)


def densify(ary):
    rslt = [0] * 16
    for i in range(16):
        for j in range(16):
            rslt[i] ^= ary[i * 16 + j]
    return rslt

def calc_hash(ary):
    pos = 0
    skip = 0
    data = None
    prg = [ord(x) for x in ary] + [17, 31, 73, 47, 23]
    for p in range(64):
        (pos, skip, data) = crypt(prg, pos=pos, skip=skip, data=data)
    rslt = densify(data[pos:] + data[:pos])
    answer = ""
    for r in rslt:
        answer += format(r, "02x")
    return answer


if __name__ == "__main__":
    test_data = "3,4,1,5"
    puzzle_data = "183,0,31,146,254,240,223,150,2,206,161,1,255,232,199,88"

    (tpos, tskip, tdata) = crypt([int(t) for t in test_data.split(",")], size=5)
    print("test data: %s" % (tdata[tpos] * tdata[(tpos + 1) % 5]))

    (ppos, pskip, pdata) = crypt([int(p) for p in puzzle_data.split(",")])
    print("Part 1: %s" % (pdata[ppos] * pdata[(ppos + 1) % 256]))

    dense = {
        "": "a2582a3a0e66e6e86e3812dcb672a272",
        "AoC 2017": "33efeb34ea91902bb2f59c9920caa6cd",
        "1,2,3": "3efbe78a8d82f29979031a4aa0b16a9d",
        "1,2,4": "63960835bcdc130f0b66d7ff4f6a5a8e"
    }

    for d in dense:
        answer = calc_hash(d)
        if answer != dense[d]:
            print("Input string '%s'\n\tgot '%s'\n\texpecting '%s'" % (d, answer, dense[d]))
            break
        else:
            print("string '%s' validated" % d)

    print("Part 2: %s" % calc_hash(puzzle_data))
