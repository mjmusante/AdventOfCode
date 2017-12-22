#! /usr/bin/python

import re
import sys

from copy import copy


class Spell:
    SHIELD = 7
    MANA = 101
    POISON = 3

    def __init__(self, name, cost, damage=0, armour=0,
                 poison=0, recharge=0, heal=0):
        self.armour = armour
        self.cost = cost
        self.damage = damage
        self.heal = heal
        self.name = name
        self.poison = poison
        self.recharge = recharge

    def __repr__(self):
        return self.name


class Tome:
    MAGIC_MISSILE = Spell("Magic Missile", 53, damage=4)
    DRAIN = Spell("Drain", 73, damage=2, heal=2)
    SHIELD = Spell("Shield", 113, armour=6)
    POISON = Spell("Poison", 173, poison=6)
    RECHARGE = Spell("Recharge", 229, recharge=5)

    book = [MAGIC_MISSILE, DRAIN, SHIELD, POISON, RECHARGE]


class GameMaster:

    def __init__(self, hp=10, mana=250, bosshp=13, bosshit=8, quiet=True):
        self.hp = hp
        self.mana = mana
        self.bosshp = bosshp
        self.bosshit = bosshit
        self.shield = 0
        self.quiet = quiet
        self.level = 0
        self.boss_poisoned = False

        self.effects = []
        self.spell_seq = []

    def player_hp(self):
        return self.hp

    def player_dead(self):
        return self.hp <= 0

    def boss_dead(self):
        return self.bosshp <= 0

    def player_mana(self):
        return self.mana

    def boss_hp(self):
        return self.bosshp

    def generate_moves(self):
        rslt = []
        for s in Tome.book:
            if s.cost < self.mana:
                rslt.append(s)

        for e in self.effects:
            if e.armour > 1 and Tome.SHIELD in rslt:
                rslt.remove(Tome.SHIELD)
            if e.poison > 1 and Tome.POISON in rslt:
                rslt.remove(Tome.POISON)
            if e.recharge > 1 and Tome.RECHARGE in rslt:
                rslt.remove(Tome.RECHARGE)

        return rslt

    def player_heal(self, amt):
        self.hp += amt

    def spend_mana(self, amt):
        self.mana -= amt

    def hit_boss(self, amt):
        self.bosshp -= amt

    def start_of_turn(self, hardmode=False):
        newhp = self.hp
        if hardmode:
            newhp -= 1

        rslt = GameMaster(newhp, self.mana, self.bosshp, self.bosshit,
                          self.quiet)
        rslt.level = self.level + 1
        if rslt.hp < 1:
            if not self.quiet:
                print("*** hardmode: player dies ***\n")
            return rslt

        for e in self.effects:
            s = copy(e)

            if s.armour > 0:
                s.armour -= 1
                if s.armour > 0:
                    rslt.effects.append(s)
                rslt.shield = Spell.SHIELD

            if s.recharge > 0:
                rslt.mana += Spell.MANA
                s.recharge -= 1
                if s.recharge > 0:
                    rslt.effects.append(s)

            if s.poison > 0:
                rslt.bosshp -= Spell.POISON
                s.poison -= 1
                if s.poison > 0:
                    rslt.effects.append(s)

        return rslt

    def turn_desc(self, who):
        if self.quiet:
            return

        spellinfo = []
        armour = 0
        for s in self.effects:
            if s.recharge > 0:
                spellinfo.append("Recharge provides %s mana; "
                                 "its timer is now %s." %
                                 (Spell.MANA, s.recharge - 1))
            if s.poison > 0:
                spellinfo.append("Poison deals %s damage, "
                                 "its timer is now %s." %
                                 (Spell.POISON, s.poison - 1))
            if s.armour > 0:
                spellinfo.append("Shield's timer is now %s" % s.armour)
                armour += Spell.SHIELD

        print("-- %s turn --" % who)
        print("- Player has %s hit points, %s armour, %s mana" %
              (self.hp, armour, self.mana))
        print("- Boss has %s hit points" % self.bosshp)
        for i in spellinfo:
            print(i)

    def cast(self, spell, desc=False, hardmode=False):

        rslt = self.start_of_turn(hardmode)
        if rslt.player_dead():
            return rslt

        if rslt.boss_dead():
            rslt.boss_poisoned = True
            if not self.quiet:
                print("Boss succumbs to poisoning.")
            return rslt

        if desc and not self.quiet:
            msg = "Player casts %s" % spell.name
            if spell.damage > 0:
                msg += ", dealing %s damage" % spell.damage
            if spell.armour > 0:
                msg += ", increasing armour by %s" % Spell.SHIELD
            if spell.heal > 0:
                msg += ", healing %s hit points" % spell.heal
            print("%s." % msg)

        rslt.spend_mana(spell.cost)
        rslt.player_heal(spell.heal)
        rslt.hit_boss(spell.damage)

        if spell.armour > 0 or spell.recharge > 0 or spell.poison > 0:
            for e in rslt.effects:
                if spell.armour > 0 and e.armour > 1:
                    raise Exception
                if spell.recharge > 0 and e.recharge > 1:
                    raise Exception
                if spell.poison > 0 and e.poison > 1:
                    raise Exception

            rslt.effects.append(spell)

        return rslt

    def boss_move(self, desc=False):
        rslt = self.start_of_turn()
        if rslt.boss_hp() <= 0:
            if desc and not self.quiet:
                print("This kills the boss, and the player wins.")
            return rslt

        hit_for = max(1, rslt.bosshit - rslt.shield)
        if desc and not self.quiet:
            print("Boss attacks for %s damage" % hit_for)
            if hit_for >= rslt.hp:
                print("*** Boss Slays Player ***")
            print("\n")

        rslt.hp -= hit_for
        return rslt

    def play(self, moves):
        self.total_cost = 0
        gm = self
        for m in moves:
            gm.turn_desc("Player")
            self.total_cost += m.cost
            gm = gm.cast(m, True)
            if gm.player_hp() <= 0 or gm.boss_hp() <= 0:
                break
            if not self.quiet:
                print("")
            gm.turn_desc("Boss")
            gm = gm.boss_move(True)
            if gm.player_hp() <= 0 or gm.boss_hp() <= 0:
                break
        return gm

    def one_move(self, spell, hardmode=False):
        self.turn_desc("Player depth %s" % self.level)
        newgm = self.cast(spell, desc=True, hardmode=hardmode)
        if newgm.player_dead() or newgm.boss_dead():
            return newgm

        newgm.turn_desc("Boss depth %s" % newgm.level)
        return newgm.boss_move(True)

    def find_lowest_mana(self, lim=None, hardmode=False):
        moves = self.generate_moves()
        if len(moves) == 0:
            return None
        best = None
        for m in moves:
            if best is not None and (best < m.cost):
                continue
            if lim is not None and lim < m.cost:
                continue
            gm = self.one_move(m, hardmode)
            if gm.player_dead():
                continue
            if gm.boss_dead():
                if gm.boss_poisoned:
                    actual_cost = 0
                else:
                    actual_cost = m.cost
                    self.spell_seq = [m]
                if best is None or best > actual_cost:
                    best = actual_cost
            else:
                if lim is None:
                    newlim = best
                else:
                    newlim = lim - m.cost
                cost = gm.find_lowest_mana(newlim, hardmode)
                if cost is not None:
                    cost += m.cost
                    if best is None or best > cost:
                        best = cost
                        self.spell_seq = [m] + gm.spell_seq
        return best


def read_boss_stats():
    lines = [line.strip() for line in open("day22.txt")]
    re_hp = re.search("Hit Points: (\d+)$", lines[0]).group(1)
    re_dam = re.search("Damage: (\d+)$", lines[1]).group(1)
    return (int(re_hp), int(re_dam))


if __name__ == "__main__":
    (bosshp, bosshit) = read_boss_stats()
    gm = GameMaster(hp=50, mana=500, bosshp=bosshp,
                    bosshit=bosshit, quiet=True)
    print("Part 1: %s" % gm.find_lowest_mana())
    print("Part 2: %s" % gm.find_lowest_mana(hardmode=True))
