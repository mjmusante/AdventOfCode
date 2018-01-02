import unittest

from day18 import Rogue


class TestRogue(unittest.TestCase):

    def test_can_create_rogue_process(self):
        r = Rogue()
        self.assertIsNotNone(r)

    def test_can_generate_next_row(self):
        r = Rogue("..^^.")
        self.assertEquals(r.next_row(), ".^^^^")

    def test_new_first_based_on_first_two_safe(self):
        r = Rogue("..")
        self.assertEquals(r.next_char_at(0), ".")

    def test_new_first_based_on_first_two_traps(self):
        r = Rogue("^^")
        self.assertEquals(r.next_char_at(0), "^")

    def test_new_first_based_on_trap_and_safe(self):
        r = Rogue("^.")
        self.assertEquals(r.next_char_at(0), ".")

    def test_new_first_based_on_safe_and_trap(self):
        r = Rogue(".^")
        self.assertEquals(r.next_char_at(0), "^")

    def test_can_cycle_first_example(self):
        r = Rogue("..^^.")
        r = Rogue(r.next_row())
        r = Rogue(r.next_row())
        self.assertEquals(r.row, "^^..^")

    def test_can_cycle_second_example(self):
        r = Rogue(".^^.^.^^^^")
        for i in range(9):
            r = Rogue(r.next_row())
        self.assertEquals("^^.^^^..^^", r.row)

    def test_can_count_safe_spaces(self):
        r = Rogue(".^^.^.^^^^")
        self.assertEquals(r.safe_count, 3)

    def test_can_self_cycle_with_safe_count(self):
        r = Rogue("..^^.")
        r.cycle(2)
        self.assertEquals(r.safe_count, 6)

    def test_can_run_large_example(self):
        r = Rogue(".^^.^.^^^^")
        r.cycle(9)
        self.assertEquals(r.safe_count, 38)

    def test_can_solve_part_1(self):
        r = Rogue(".^^^^^.^^^..^^^^^...^.^..^^^.^^....^.^...^^^...^^^^..^...^."
                  "..^^.^.^.......^..^^...^.^.^^..^^^^^...^.")
        r.cycle(39)
        self.assertEquals(r.safe_count, 1956)

    def test_can_solve_part_2(self):
        r = Rogue(".^^^^^.^^^..^^^^^...^.^..^^^.^^....^.^...^^^...^^^^..^...^."
                  "..^^.^.^.......^..^^...^.^.^^..^^^^^...^.")
        r.cycle(399999)
        self.assertEquals(r.safe_count, 19995121)
