import unittest

from copy import copy

from day22 import GameMaster, Spell, Tome, read_boss_stats


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

    def test_cannot_stack_sheilds(self):
        s = Spell("Dual Armour", 2, armour=5)
        gm = GameMaster(hp=10)

        newgm = gm.cast(s)
        bossgm = newgm.boss_move()
        self.assertEqual(bossgm.shield, 7)

        with self.assertRaises(Exception):
            newgm = bossgm.cast(s)

        self.assertEqual(bossgm.shield, 7)

    def test_can_shield_from_boss(self):
        s = Spell("Armour", 0, armour=6)
        gm = GameMaster(hp=10, bosshit=12)
        newgm = gm.cast(s)
        bossgm = newgm.boss_move()
        self.assertEqual(bossgm.player_hp(), 5)
        self.assertEqual(bossgm.shield, 7)

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
        gm = GameMaster(hp=10, mana=250, bosshp=13, bosshit=8)
        gm.play([Tome.POISON, Tome.MAGIC_MISSILE])
        self.assertEqual(gm.total_cost,
                         Tome.POISON.cost + Tome.MAGIC_MISSILE.cost)

    def test_can_play_second_example(self):
        gm = GameMaster(hp=10, mana=250, bosshp=14, bosshit=8)
        gm.play([Tome.RECHARGE, Tome.SHIELD, Tome.DRAIN, Tome.POISON,
                 Tome.MAGIC_MISSILE])
        self.assertEqual(gm.total_cost,
                         Tome.RECHARGE.cost + Tome.SHIELD.cost +
                         Tome.DRAIN.cost + Tome.POISON.cost +
                         Tome.MAGIC_MISSILE.cost)

    def test_can_detect_player_death(self):
        gm = GameMaster(hp=2)
        newgm = gm.one_move(Tome.MAGIC_MISSILE)
        self.assertTrue(newgm.player_dead())

    def test_can_detect_boss_death(self):
        gm = GameMaster(bosshp=2)
        newgm = gm.one_move(Tome.MAGIC_MISSILE)
        self.assertTrue(newgm.boss_dead())

    def test_can_run_first_scenario(self):
        gm = GameMaster(hp=10, mana=250, bosshp=13, bosshit=8)
        self.assertEqual(gm.find_lowest_mana(),
                         Tome.POISON.cost + Tome.MAGIC_MISSILE.cost)

    def test_can_run_second_scenario(self):
        gm = GameMaster(hp=10, mana=250, bosshp=14, bosshit=8)
        self.assertEqual(gm.find_lowest_mana(),
                         Tome.RECHARGE.cost + Tome.SHIELD.cost +
                         Tome.DRAIN.cost + Tome.POISON.cost +
                         Tome.MAGIC_MISSILE.cost)

    def test_will_not_generate_sheild_if_already_active(self):
        gm = GameMaster()
        moves = gm.generate_moves()
        self.assertIn(Tome.SHIELD, moves)
        newgm = gm.cast(Tome.SHIELD)
        newmoves = newgm.generate_moves()
        self.assertNotIn(Tome.SHIELD, newmoves)

    def test_will_not_generate_recharge_if_already_active(self):
        gm = GameMaster()
        moves = gm.generate_moves()
        self.assertIn(Tome.RECHARGE, moves)
        newgm = gm.cast(Tome.RECHARGE)
        newmoves = newgm.generate_moves()
        self.assertNotIn(Tome.RECHARGE, newmoves)

    def test_can_start_new_effect_at_end_of_old_effect(self):
        gm = GameMaster()
        moves = gm.generate_moves()
        self.assertIn(Tome.SHIELD, moves)
        newgm = gm.cast(Tome.SHIELD)
        e = copy(newgm.effects[0])
        e.armour = 1
        newgm.effects[0] = e
        newmoves = newgm.generate_moves()
        self.assertIn(Tome.SHIELD, newmoves)

    def test_can_read_boss_settings(self):
        (hp, dam) = read_boss_stats()
        self.assertEqual(hp, 51)
        self.assertEqual(dam, 9)

    def test_can_solve_part_1(self):
        (bosshp, bosshit) = read_boss_stats()
        gm = GameMaster(hp=50, mana=500, bosshp=bosshp,
                        bosshit=bosshit, quiet=True)
        self.assertEqual(gm.find_lowest_mana(), 900)
