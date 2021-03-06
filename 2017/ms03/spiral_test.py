import unittest

from spiral import coords_of, sum_larger_than


class TestSpiral(unittest.TestCase):

    def test_steps_from_1(self):
        x, y = coords_of(1)
        self.assertEquals(int(abs(x) + abs(y)), 0)

    def test_steps_from_12(self):
        x, y = coords_of(12)
        self.assertEquals(int(abs(x) + abs(y)), 3)

    def test_steps_from_23(self):
        x, y = coords_of(23)
        self.assertEquals(int(abs(x) + abs(y)), 2)

    def test_steps_from_31(self):
        x, y = coords_of(1024)
        self.assertEquals(int(abs(x) + abs(y)), 31)

    def test_larger(self):
        self.assertEquals(sum_larger_than(7), 10)
