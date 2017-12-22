import unittest

from gen import judge, judge_gen


class TestGen(unittest.TestCase):

    def test_first_example(self):
        self.assertEquals(judge(65, 8921, 5), 1)

    def test_second_example(self):
        self.assertEquals(judge(65, 8921), 588)

    def test_third_example(self):
        self.assertEquals(judge_gen(65, 8921), 309)

    def test_can_solve_part_1(self):
        self.assertEquals(judge(703, 516), 594)

    def test_can_solve_part_2(self):
        self.assertEquals(judge_gen(703, 516), 328)
