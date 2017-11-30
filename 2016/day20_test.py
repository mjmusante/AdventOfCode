import unittest

from day20 import SpaceMap

class TestSpaceMap(unittest.TestCase):

    def test_can_create_range(self):
        sm = SpaceMap(maxval=32)
        self.assertEqual(sm.in_map(10), True)
        self.assertEqual(sm.in_map(64), False)

    def test_range_limits(self):
        sm = SpaceMap(10, 20)
        self.assertEqual(sm.in_map(9), False)
        self.assertEqual(sm.in_map(10), True)
        self.assertEqual(sm.in_map(20), True)
        self.assertEqual(sm.in_map(21), False)

    def test_can_remove_range(self):
        sm = SpaceMap(maxval=32)
        sm.remove_range(7, 14)
        self.assertEqual(sm.in_map(6), True)
        self.assertEqual(sm.in_map(7), False)
        self.assertEqual(sm.in_map(14), False)
        self.assertEqual(sm.in_map(15), True)