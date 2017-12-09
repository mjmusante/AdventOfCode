def next_char(stuff):
    pos = 0
    while pos < len(stuff):
        while stuff[pos] == "!":
            pos += 2
        yield stuff[pos]
        pos += 1


def score(str):
    comment = False
    rslt = 0
    level = 0
    chars = 0
    for c in next_char(str):
        if comment:
            if c == ">":
                comment = False
            else:
                chars += 1
        else:
            if c == "<":
                comment = True
            elif c == "{":
                level += 1
            elif c == "}" and level > 0:
                rslt += level
                level -= 1
    return (rslt, chars)

testdata = {
    "{}" : (1, 0),
    "{{{}}}": (6, 0),
    "{{},{}}": (5, 0),
    "{{{},{},{{}}}}": (16, 0),
    "{<a>,<a>,<a>,<a>}": (1, 4),
    "{{<ab>},{<ab>},{<ab>},{<ab>}}": (9, 8),
    "{{<!!>},{<!!>},{<!!>},{<!!>}}": (9, 0),
    "{{<a!>},{<a!>},{<a!>},{<ab>}}": (3, 17),
    "{}}}}}": (1, 0),
    "{<!!!>{}>}": (1, 2)
}

tc = 0
for t in testdata:
    tc += 1
    answer = score(t)
    if (answer[0] != testdata[t][0]):
        print("'%s' expecting %s, got %s" % (t, testdata[t][0], answer[0]))
    if (answer[1] != testdata[t][1]):
        print("'%s' expecting %s garbage, got %s" % (t, testdata[t][1],
              answer[1]))

foo = [line.strip() for line in open("puzzle_data.txt")]
print("Part 1: %s\nPart 2: %s" % score(foo[0]))
