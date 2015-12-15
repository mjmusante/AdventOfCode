#! /usr/bin/python

class Deliverer:
    def __init__(self):
        self.x = 0
        self.y = 0

    def move(self, direction, houses):
        if direction == '^':
            self.y -= 1
        elif direction == 'v':
            self.y += 1
        elif direction == '<':
            self.x -= 1
        elif direction == '>':
            self.x += 1
        self.deliver(houses)

    def deliver(self, houses):
        if (self.x, self.y) in houses:
            houses[(self.x, self.y)] += 1
        else:
            houses[(self.x, self.y)] = 1

with open("day3.txt", "r") as f:
    data = f.read().strip()

# This year's santa
santa = Deliverer()

# Next year has santa and robosanta
nextsanta = Deliverer()
robosanta = Deliverer()

# the infinite grid
thisyear = {}
nextyear = {}

santa.deliver(thisyear)

nextsanta.deliver(nextyear)
robosanta.deliver(nextyear)

is_robo = False
for c in data:
    # This year, santa moves every time
    santa.move(c, thisyear)

    # Next year, santa and robo-santa alternate
    if is_robo:
        robosanta.move(c, nextyear)
    else:
        nextsanta.move(c, nextyear)
    is_robo = not is_robo

print("Number of houses visted this year: %s" % len(thisyear))
print("Number of houses visted next year: %s" % len(nextyear))
