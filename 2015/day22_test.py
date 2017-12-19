import unittest

from day22 import GameMaster, Spell, Tome


class TestMagicCode(unittest.TestCase):

    def test_can_generate_all_available_spells(self):
        gm = GameMaster()
        lst = gm.generate_moves()
        self.assertEquals(len(lst), 5)

    def test_can_generate_only_some_spells(self):
        gm = GameMaster(mana=100)
        lst = gm.generate_moves()
        self.assertEquals(len(lst), 2)

    def test_can_generate_magic_missile(self):
        gm = GameMaster()
        moves = gm.generate_moves()
        self.assertEquals(moves[0].name, "Magic Missile")

    def test_can_perform_damage_move(self):
        s = Spell("Damage", 1, damage=1)
        gm = GameMaster(mana=5, bosshp=5)
        newgm = gm.cast(s)
        self.assertEqual(newgm.player_mana(), 4)
        self.assertEqual(newgm.boss_hp(), 4)

    def test_can_perform_boss_move(self):
        gm = GameMaster(hp=10, bosshit=7)
        newgm = gm.boss_move()
        self.assertEqual(newgm.player_hp(), 3)

    def test_can_perform_heal_move(self):
        s = Spell("Heal", 0, heal=2)
        gm = GameMaster(hp=5)
        newgm = gm.cast(s)
        self.assertEqual(newgm.player_hp(), 7)

    def test_can_linger_effects(self):
        s = Spell("Recharge", 1, recharge=2)
        gm = GameMaster(mana=10)
        newgm = gm.cast(s)
        self.assertEqual(newgm.player_mana(), 9)
        newgm = newgm.boss_move()
        self.assertEqual(newgm.player_mana(), 9 + Spell.MANA)

    def test_can_stack_sheilds(self):
        s = Spell("Dual Armour", 2, armour=5)
        gm = GameMaster(hp=10)

        newgm = gm.cast(s)
        newgm = newgm.cast(s)
        self.assertEqual(newgm.shield, 7)

        newgm = newgm.boss_move()
        self.assertEqual(newgm.shield, 14)

    def test_can_shield_from_boss(self):
        s = Spell("Armour", 0, armour=6)
        gm = GameMaster(hp=10, bosshit=12)
        newgm = gm.cast(s)
        bossgm = newgm.boss_move()
        self.assertEqual(bossgm.player_hp(), 5)

    def test_boss_hits_for_at_least_one_point(self):
        s = Spell("Armour", 0, armour=3)
        gm = GameMaster(hp=10, bosshit=4)
        newgm = gm.cast(s)
        bossgm = newgm.boss_move()
        self.assertEqual(bossgm.player_hp(), 9)

    def test_poison(self):
        s = Spell("Poison", 0, poison=3)
        gm = GameMaster(hp=10, bosshp=10)
        newgm = gm.cast(s)
        self.assertEqual(newgm.bosshp, 10)
        bossgm = newgm.boss_move()
        self.assertEqual(bossgm.bosshp, 10 - Spell.POISON)

    def test_can_play_first_example(self):
        gm = GameMaster(hp=10, mana=250, bosshp=13, bosshit=8, quiet=True)
        gm.play([Tome.POISON, Tome.MAGIC_MISSILE])
        self.assertEqual(gm.total_cost,
                         Tome.POISON.cost + Tome.MAGIC_MISSILE.cost)

    def test_can_play_second_example(self):
        gm = GameMaster(hp=10, mana=250, bosshp=14, bosshit=8, quiet=True)
        gm.play([Tome.RECHARGE, Tome.SHIELD, Tome.DRAIN, Tome.POISON,
                 Tome.MAGIC_MISSILE])
        self.assertEqual(gm.total_cost,
                         Tome.RECHARGE.cost + Tome.SHIELD.cost +
                         Tome.DRAIN.cost + Tome.POISON.cost +
                         Tome.MAGIC_MISSILE.cost)
