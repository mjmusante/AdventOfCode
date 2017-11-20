import unittest

from day13 import FirstFloor

class Day13Test(unittest.TestCase):

    def test_can_create_floor(self):
        ff = FirstFloor(10)
        self.assertEqual(ff.designer, 10)

    def test_1_1_has_space(self):
        ff = FirstFloor(10)
        self.assertEqual(ff.get(1, 1), FirstFloor.SPACE)

    def test_0_1_has_wall(self):
        ff = FirstFloor(10)
        self.assertEqual(ff.get(1, 0), FirstFloor.WALL)

    def test_adjacent_spaces(self):

        def check_validity(ff, x, y, valid):
            for node in ff.adjacent_spaces(x, y):
                self.assertIn(node, valid)
                valid.remove(node)
            self.assertEqual(len(valid), 0)


        ff = FirstFloor(10)
        check_validity(ff, 1, 1, [(0, 1), (1, 2)])
        check_validity(ff, 6, 5, [(5, 5), (6, 4), (7, 5), (6, 6)])

    def test_sample_data(self):
        ff = FirstFloor(10)
        self.assertEqual(len(ff.find_shortest_path((1, 1), (7, 4))) - 1, 11)