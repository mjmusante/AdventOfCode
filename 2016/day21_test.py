import unittest

from day21 import Hasher


class TestDay21(unittest.TestCase):

    def test_create(self):
        x = Hasher()

    def test_hasher_string(self):
        x = Hasher("abcde")
        self.assertEqual(x.get_hash(), "abcde")

    def test_swap_position(self):
        x = Hasher("abcde")
        x.execute(["swap position 0 with position 2"])
        self.assertEqual(x.get_hash(), "cbade")

    def test_swap_letter(self):
        x = Hasher("abcde")
        x.execute(["swap letter b with letter d"])
        self.assertEqual(x.get_hash(), "adcbe")

    def test_rotate(self):
        x = Hasher("abcde")
        x.execute(["rotate left 2 step"])
        self.assertEqual(x.get_hash(), "cdeab")
        x.execute(["rotate right 3 steps"])
        self.assertEqual(x.get_hash(), "eabcd")

    def test_rotate_larger_than_string(self):
        x = Hasher("abcde")
        x.execute(["rotate right 6 steps"])
        self.assertEqual(x.get_hash(), "eabcd")

    def test_rotate_pos(self):
        x = Hasher("abcde")
        x.execute(["rotate based on position of letter d"])
        self.assertEqual(x.get_hash(), "bcdea")

        x = Hasher("abcdefghijkl")
        x.execute(["rotate based on position of letter g"])
        self.assertEqual(x.get_hash(), "efghijklabcd")

    def test_reverse(self):
        x = Hasher("abcde")
        x.execute(["reverse positions 1 through 4"])
        self.assertEqual(x.get_hash(), "aedcb")

    def test_reverse_0(self):
        x = Hasher("abcde")
        x.execute(["reverse positions 0 through 4"])
        self.assertEqual(x.get_hash(), "edcba")

    def test_move(self):
        x = Hasher("abcde")
        x.execute(["move position 1 to position 3"])
        self.assertEqual(x.get_hash(), "acdbe")

    def test_broken_example(self):
        x = Hasher("ecabd")
        x.execute(["rotate based on position of letter d"])
        self.assertEqual(x.get_hash(), "decab")

    def test_example(self):
        x = Hasher("abcde")
        x.execute([
            "swap position 4 with position 0",
            "swap letter d with letter b",
            "reverse positions 0 through 4",
            "rotate left 1 step",
            "move position 1 to position 4",
            "move position 3 to position 0",
            "rotate based on position of letter b",
            "rotate based on position of letter d",
        ])
        self.assertEqual(x.get_hash(), "decab")
