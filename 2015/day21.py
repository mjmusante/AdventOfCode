#! /usr/bin/python

import itertools

# Weapons:    Cost  Damage  Armor
# Dagger        8     4       0
# Shortsword   10     5       0
# Warhammer    25     6       0
# Longsword    40     7       0
# Greataxe     74     8       0
# 
# Armor:      Cost  Damage  Armor
# Leather      13     0       1
# Chainmail    31     0       2
# Splintmail   53     0       3
# Bandedmail   75     0       4
# Platemail   102     0       5
# 
# Rings:      Cost  Damage  Armor
# Damage +1    25     1       0
# Damage +2    50     2       0
# Damage +3   100     3       0
# Defense +1   20     0       1
# Defense +2   40     0       2
# Defense +3   80     0       3

#
# Hit Points: 103
# Damage: 9
# Armor: 2

class Item:
    def __init__(self, name, cost, dam, prot):
        self.name = name
        self.cost = cost
        self.dam = dam
        self.prot = prot


weapons = [
        Item("Dagger", 8, 4, 0),
        Item("Shortsword", 10, 5, 0),
        Item("Warhammer", 25, 6, 0),
        Item("Longsword", 40, 7, 0),
        Item("Greataxe", 74, 8, 0)
        ]

armour = [
        Item("Nothing", 0, 0, 0),
        Item("Leather", 13, 0, 1),
        Item("Chainmail", 31, 0, 2),
        Item("Splitmail", 53, 0, 3),
        Item("Bandedmail", 75, 0, 4),
        Item("Platemail", 102, 0, 5)
        ]

rings = [
        Item("Damage +1", 25, 1, 0),
        Item("Damage +2", 50, 3, 0),
        Item("Damage +3", 100, 3, 0),
        Item("Defense +1", 20, 0, 1),
        Item("Defense +2", 40, 0, 2),
        Item("Defense +3", 80, 0, 3)
        ]

class Creature:
    def __init__(self, name, hp, dam, prot):
        self.name = name
        self.hp = hp
        self.dam = dam
        self.prot = prot

def battle(p1, p2):
    while True:
        d1 = p1.dam - p2.prot
        p2.hp -= d1
        if p2.hp <= 0:
            # print("%s does %s damage; %s dies" % (p1.name, d1, p2.name))
            return 2
#       print("%s does %s damage; %s goes down to %s hp" %
#               (p1.name, d1, p2.name, p2.hp))
        d2 = p2.dam - p1.prot
        p1.hp -= d2
        if p1.hp <= 0:
            # print("%s does %s damage; %s dies" % (p2.name, d2, p1.name))
            return 1
#       print("%s does %s damage; %s goes down to %s hp" %
#               (p2.name, d2, p1.name, p1.hp))


a = Creature("Player", 8, 5, 5)
b = Creature("Boss", 12, 7, 2)
battle(a, b)

rlist = []
for i in range(0, 3):
    for j in [x for x in itertools.combinations(rings, i)]:
        rlist.append(j)

best = 1000000
worst = 0
for w in weapons:
    for a in armour:
        for rl in rlist:
            dam = w.dam + a.dam
            prot = w.prot + a.prot
            cost = w.cost + a.cost

            for r in rl:
                dam += r.dam
                prot += r.prot
                cost += r.cost

            b = Creature("Boss", 103, 9, 2)
            p = Creature("Player", 100, dam, prot)
            if battle(b, p) == 1:
                if cost < best:
                    best = cost
            else:
                if cost > worst:
                    worst = cost

print(best)
print(worst)
