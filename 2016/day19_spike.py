

def rotate(ary):
    i = 0
    orig = len(ary)
    itr = 0
    while len(ary) > 1:
        n = (i + 1) % len(ary)
        ary[i] = (ary[i][0], ary[i][1] + ary[n][1])
        ary.remove(ary[n])
        i = (i + 1) % len(ary)
        itr += 1
        if itr % 10000 == 0:
            print("%s items left to go after %s iterations" % (len(ary), itr))
    print("%s: %s" % (orig, ary))



for lim in range(1, 20):
    l = []
    for i in range(lim):
        l.append((i + 1, 1),)
    rotate(l)