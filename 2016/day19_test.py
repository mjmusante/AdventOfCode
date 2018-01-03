import unittest

from day19 import pattern, circular_josephus


class TestJosephus(unittest.TestCase):

    def test_pattern_matches_calculation(self):
        for i in range(1, 500):
            self.assertEquals(pattern(i), circular_josephus(i))
