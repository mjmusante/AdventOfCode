import unittest

from spinlock import do_spin


class TestSpinlock(unittest.TestCase):

    def test_example_problem(self):
        rslt = do_spin(3, 2017)
        self.assertEquals(rslt[rslt.index(2017) + 1], 638)
