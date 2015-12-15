#! /usr/bin/python

class Summer:
    def __init__(self, values, limit):
        self.values = values
        self.limit = limit
        self.cur = []
        for i in range(0, values):
            self.cur.append(1)

    def inc_one(self):
        maxval = self.limit - self.values + 1
        for i in range(1, self.values):
            if self.cur[i] < maxval:
                self.cur[i] += 1
                break
            if i + 1 == self.values:
                return False
            self.cur[i] = 1
        return True

    def get_next(self):
        tot = sum(self.cur)
        if tot < self.limit:
            # must be in here for the first time
            self.cur[0] = self.limit - tot + 1
            return self.cur
        self.cur[0] = 0
        if not self.inc_one():
            return None
        while sum(self.cur) >= self.limit:
            if not self.inc_one():
                return None
        self.cur[0] = self.limit - sum(self.cur)
        return (self.cur)

class Ingredient:
    def __init__(self, name, ilist):
        self.name = name
        self.feature = []

        feature = ilist.split(", ")
        for f in feature:
            ftype, fvalue = f.split(" ")
            if ftype != "calories":
                self.feature.append(int(fvalue))
            else:
                self.calories = int(fvalue)

s = Summer(3, 7)
x = s.get_next()
while x:
    if sum(x) != 7:
        print("The list %s does not sum to 7" % x)
        sys.exit(0)
    x = s.get_next()

s = Summer(4, 100)
x = s.get_next()
while x:
    if 0 in x:
        print("Illegal list %s" % x)
        sys.exit(1)
    x = s.get_next()

test_case = """Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
""".splitlines()

with open("day15.txt") as f:
    lines = f.read().splitlines()

ings = []
for l in lines:
    ings.append(Ingredient(*l.split(": ")))

fcount = len(ings[0].feature)

maxcookie = 1
max500 = 1
s = Summer(len(ings), 100)
x = s.get_next()
while x:
    val = 1
    for i in range(0, fcount):
        tot = 0
        for j in range(0, len(ings)):
            amount = ings[j].feature[i] * x[j]
            tot += amount
        if tot <= 0:
            val = 1
            break
        val *= tot
    if val > maxcookie:
        maxcookie = val
        savedx = list(x)
    cals = 0
    for j in range(0, len(ings)):
        cals += ings[j].calories * x[j]
    if cals == 500 and val > max500:
        max500 = val
    x = s.get_next()

print("Best cookie: %s" % maxcookie)
print("Best 500-calorie cookie: %s" % max500)
