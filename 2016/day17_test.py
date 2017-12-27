import unittest

from day17 import Vault


class TestVault(unittest.TestCase):

    def test_can_create_vault(self):
        v = Vault("hijkl")
        self.assertIsNotNone(v)

    def test_starts_at_0_0(self):
        v = Vault("hijkl")
        self.assertEquals(v.loc(), (0, 0))

    def test_generates_first_moves(self):
        v = Vault("hijkl")
        m = v.get_moves()
        self.assertEquals(m, ['d'])

    def test_can_move_down(self):
        v = Vault("hijkl")
        v = v.move_down()
        self.assertEquals(v.loc(), (0, 1))

    def test_moving_down_generates_new_moves(self):
        v = Vault("hijkl")
        v = v.move_down()
        self.assertEquals(v.get_moves(), ['u', 'r'])

    def test_moving_down_and_right_returns_no_open_doors(self):
        v = Vault("hijkl")
        v = v.move_down().move_right()
        self.assertEquals(v.get_moves(), [])

    def test_moving_down_and_up_reveals_right_door_open(self):
        v = Vault("hijkl")
        v = v.move_down().move_up()
        self.assertEquals(v.get_moves(), ['r'])

    def test_moving_counts_moves(self):
        v = Vault("hijkl")
        v = v.move_down().move_up().move_right()
        self.assertEquals(v.mc, 3)

    def test_shortest_for_example_1(self):
        v = Vault("ihgpwlah")
        self.assertEquals(v.find_shortest_path().direc, "DDRRRD")

    def test_shortest_for_example_2(self):
        v = Vault("kglvqrro")
        self.assertEquals(v.find_shortest_path().direc, "DDUDRLRRUDRD")

    def test_shortest_for_example_3(self):
        v = Vault("ulqzkmiv")
        self.assertEquals(v.find_shortest_path().direc,
                          "DRURDRUDDLLDLUURRDULRLDUUDDDRR")

    def test_longest_for_example_1(self):
        v = Vault("ihgpwlah")
        self.assertEquals(v.find_longest_path().mc, 370)

    def test_longest_for_example_2(self):
        v = Vault("kglvqrro")
        self.assertEquals(v.find_longest_path().mc, 492)

    def test_longest_for_example_3(self):
        v = Vault("ulqzkmiv")
        self.assertEquals(v.find_longest_path().mc, 830)

    def test_can_solve_part_1(self):
        v = Vault("hhhxzeay")
        self.assertEquals(v.find_shortest_path().direc, "DDRUDLRRRD")

    def test_can_solve_part_2(self):
        v = Vault("hhhxzeay")
        self.assertEquals(v.find_longest_path().mc, 398)
