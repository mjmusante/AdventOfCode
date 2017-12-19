#! /usr/bin/python

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


class Tome:
    MAGIC_MISSILE = Spell("Magic Missile", 53, damage=4)
    DRAIN = Spell("Drain", 73, damage=2, heal=2)
    SHIELD = Spell("Shield", 113, armour=5)
    POISON = Spell("Poison", 173, poison=6)
    RECHARGE = Spell("Recharge", 229, recharge=5)

    book = [MAGIC_MISSILE, DRAIN, SHIELD, POISON, RECHARGE]


class GameMaster:

    def __init__(self, hp=10, mana=250, bosshp=13, bosshit=8, quiet=False):
        self.hp = hp
        self.mana = mana
        self.bosshp = bosshp
        self.bosshit = bosshit
        self.shield = 0
        self.quiet = quiet

        self.effects = []

    def player_hp(self):
        return self.hp

    def player_mana(self):
        return self.mana

    def boss_hp(self):
        return self.bosshp

    def generate_moves(self):
        rslt = []
        for s in Tome.book:
            if s.cost < self.mana:
                rslt.append(s)
        return rslt

    def player_heal(self, amt):
        self.hp += amt

    def spend_mana(self, amt):
        self.mana -= amt

    def hit_boss(self, amt):
        self.bosshp -= amt

    def start_of_turn(self):
        rslt = GameMaster(self.hp, self.mana, self.bosshp, self.bosshit,
                          self.quiet)
        for e in self.effects:
            s = copy(e)

            if s.armour > 0:
                s.armour -= 1
                rslt.effects.append(s)
                rslt.shield += Spell.SHIELD

            if s.recharge > 0:
                rslt.mana += Spell.MANA
                s.recharge -= 1
                rslt.effects.append(s)

            if s.poison > 0:
                rslt.bosshp -= Spell.POISON
                s.poison -= 1
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
                spellinfo.append("Sheild's timer is now %s" % s.armour)
                armour += Spell.SHIELD

        print("-- %s turn --" % who)
        print("- Player has %s hit points, %s armour, %s mana" %
              (self.hp, armour, self.mana))
        print("- Boss has %s hit points" % self.bosshp)
        for i in spellinfo:
            print(i)

    def cast(self, spell, desc=False):

        if desc and not self.quiet:
            msg = "Player casts %s" % spell.name
            if spell.damage > 0:
                msg += ", dealing %s damage" % spell.damage
            if spell.armour > 0:
                msg += ", increasing armour by %s" % Spell.SHIELD
            if spell.heal > 0:
                msg += ", healing %s hit points" % spell.heal
            print("%s." % msg)

        rslt = self.start_of_turn()
        rslt.player_heal(spell.heal)
        rslt.spend_mana(spell.cost)
        rslt.hit_boss(spell.damage)

        if spell.armour > 0 or spell.recharge > 0 or spell.poison > 0:
            rslt.effects.append(spell)

        return rslt

    def boss_move(self, desc=False):
        rslt = self.start_of_turn()
        if rslt.boss_hp() <= 0:
            if not self.quiet:
                print("This kills the boss, and the player wins.")
            return rslt

        hit_for = max(1, rslt.bosshit - rslt.shield)
        if desc and not self.quiet:
            print("Boss attacks for %s damage\n" % hit_for)

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


if __name__ == "__main__":
    pass
