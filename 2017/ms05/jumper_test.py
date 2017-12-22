import unittest

from jumper import count_jumps, count_wacky_jumps


class TestJumper(unittest.TestCase):

    def test_count_jumps(self):
        self.assertEquals(count_jumps([0, 3, 0, 1, -3]), 5)

    def test_count_wacky_jumps(self):
        self.assertEquals(count_wacky_jumps([0, 3, 0, 1, -3]), 10)
