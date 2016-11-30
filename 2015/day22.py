#! /usr/bin/python

import sys

# boss:
# Hit Points: 51
# Damage: 9


def valid_spells(mana, shield, poison, recharge):
    splist = []
    if mana >= 53:
        splist.append("mm")
    if mana >= 73:
        splist.append("drain")
    if mana >= 113 and shield == 0:
        splist.append("shield")
    if mana >= 173 and poison == 0:
        splist.append("poison")
    if mana >= 229 and recharge == 0:
        splist.append("recharge")
    return splist

def do_magic_missile(enemy_hp, mana):
    assert mana >= 53
    return (enemy_hp - 4, mana - 53)

def do_drain(enemy_hp, hp, mana):
    assert mana >= 73, "mana=%s" % mana
    return (enemy_hp - 2, hp + 2, mana - 73)

def do_shield(mana):
    assert mana >= 113
    return (6, mana - 113)

def do_poison(mana):
    assert mana >= 173
    return (6, mana - 173)

def do_recharge(mana):
    assert mana >= 229
    return (5, mana - 229)


player_hp = 50
player_mana = 500
boss_hp = 51
boss_dam = 9

MAX_MANA = 500

def play_turn(level, mana, hp, shield, poison, recharge, enemy_hp, dam):
    global MAX_MANA

    level += 1
    assert mana >= 0

    if enemy_hp < 1:
        if MAX_MANA > mana:
            mana = MAX_MANA
        print("Enemy died with %s mana left" % mana)
        return True

    vs = valid_spells(mana, shield, poison, recharge)


    if len(vs) == 0:
        print("%s no valid spells" % ('>' * level))
        return False

    print("%s (%s,%s,%s,%s,%s,%s,%s)" % ('>' * level, mana, hp, shield, poison, recharge, enemy_hp, dam))
    print("%s (%s) %s" % ('>' * level, mana, vs))

    for i in vs:
        # apply effects
        if shield > 0:
            shield -= 1
        if poison > 0:
            if enemy_hp < 4:
                if MAX_MANA > mana:
                    mana = MAX_MANA
                print("Enemy poisoned with %s mana left" % mana)
                return True
            enemy_hp -= 3
            poison -= 1
        if recharge > 0:
            mana += 101
            recharge -= 1
        print("%s [%s/%s]" % ('>' * level, enemy_hp, mana))
        print("%s casting %s" % ('>' * level, i))

        # cast spells
        do_dam = 0
        do_hp = 0
        do_regen = 0
        mana_cost = 0

        if i == "mm":
            do_dam = 4
            mana_cost = 53
        elif i == "drain":
            do_dam = 2
            do_hp = 2
            mana_cost = 73
        elif i == "shield":
            add_shield = 6
            mana_cost = 113
        elif i == "poison":
            add_poison = 6
            mana_cost = 173
        elif i == "recharge":
            add_recharge = 5
            mana_cost = 229
        else:
            print("Unknown spell %s" % i)
            sys.exit(1)
        if enemy_hp <= 0:
            if MAX_MANA > mana:
                MAX_MANA = mana
            print("enemy dead with %s mana left" % mana)
            return True
        hp -= dam
        if shield > 0:
            hp += 7
        if hp < 1:
            return False

        play_turn(level, mana, hp, shield, poison, recharge, enemy_hp, dam)
    print("%s no more spells" % ('>' * level))

play_turn(0, 500, 50, 0, 0, 0, 51, 9)
        

sys.exit(0)




class Player:
    def __init__(self, hp, mana):
        self.hp = hp
        self.mana = mana

        self.shield = 0
        self.poison = 0
        self.recharge = 0

        self.boss_hp = 51
        self.boss_dam = 9

    def valid_spells(self):
        splist = []
        if self.mana >= 53:
            splist.append("mm")
        if self.mana >= 73:
            splist.append("drain")
        if self.mana >= 113 and self.shield == 0:
            splist.append("shield")
        if self.mana >= 173 and self.poison == 0:
            splist.append("poison")
        if self.mana >= 229 and self.recharge == 0:
            splist.append("recharge")
        return splist



def player_wins(player):
    l = player.valid_spells()
    for i in l:
        player.apply_effects()
        if player.boss_hp <= 0:
            return True
        player.cast(l)
        if player.boss_hp <= 0:
            return True
        player.boss_move()
        if player.hp <= 0:
            return False

p = Player(50, 500)

