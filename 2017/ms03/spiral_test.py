import unittest

from spiral import Spiral

class TestSpiral(unittest.TestCase):

    def test_steps_from_1(self):
        s = Spiral()
        self.assertEquals(s.steps_from_1(1), 0)
        self.assertEquals(s.steps_from_1(12), 3)
        self.assertEquals(s.steps_from_1(23), 2)
        self.assertEquals(s.steps_from_1(1024), 31)

    def test_coords(self):
        s = Spiral()
        self.assertEquals(s.coords_of(1), (0, 0))
        self.assertEquals(s.coords_of(22), (-1, -2))
        self.assertEquals(s.coords_of(13), (2, 2))

    def test_larger(self):
        s = Spiral()
        self.assertEquals(s.sum_larger_than(7), 10)

    # def test_summer(self):
    #     s = Spiral()
    #     self.assertEquals(s.sum_larger_than(1), 2)
