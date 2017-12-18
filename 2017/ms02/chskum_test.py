import unittest

from cksum import Cksum


class TestCksum(unittest.TestCase):

    def test_example(self):
        ck = Cksum(["5 1 9 5", "7 5 3", "2 4 6 8"])
        self.assertEquals(ck.cksum(), 18)

    def test_part2(self):
        ck = Cksum(["5 9 2 8", "9 4 7 3", "3 8 6 5"])
        self.assertEquals(ck.even(), 9)
