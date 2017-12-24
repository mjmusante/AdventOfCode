import unittest

from bridge import Bridge

TESTDATA = [
    "0/2",
    "2/2",
    "2/3",
    "3/4",
    "3/5",
    "0/1",
    "10/1",
    "9/10",
]


class TestBridge(unittest.TestCase):

    def test_can_create_bridge(self):
        b = Bridge()
        self.assertIsNotNone(b)

    def test_can_generate_moves_when_empty(self):
        b = Bridge(TESTDATA)
        m = b.find_moves()
        self.assertEquals(m, [[0, 2], [0, 1]])

    def test_starting_bridge_strength_is_0(self):
        b = Bridge(TESTDATA)
        self.assertEquals(b.strength(), 0)

    def test_can_attach_to_empty_bridge(self):
        b = Bridge(TESTDATA)
        b = b.attach([0, 2])
        self.assertEquals(b.bridge, [0, 2])
        self.assertNotIn(b.ports, [0, 2])

    def test_can_attach_second_entry(self):
        b = Bridge(TESTDATA)
        b = b.attach([0, 2])
        b = b.attach([2, 3])
        self.assertEquals(b.strength(), 7)
        self.assertNotIn(b.ports, [0, 2])
        self.assertNotIn(b.ports, [2, 3])

    def test_can_find_moves_from_nonempty_bridge(self):
        b = Bridge(TESTDATA)
        b = b.attach([0, 2])
        m = b.find_moves()
        self.assertEquals(m, [[2, 2], [2, 3]])

    def test_can_find_strength_of_move(self):
        b = Bridge()
        self.assertEquals(b.move_strength([4, 7]), 11)

    def test_can_solve_example(self):
        b = Bridge(TESTDATA)
        self.assertEquals(b.solve(), 31)

    def test_can_solve_second_example(self):
        b = Bridge(TESTDATA)
        self.assertEquals(b.solve_part_2(), (4, 19))

    def test_can_solve_part_1(self):
        b = Bridge(ports=[line.strip() for line in open("puzzle_data.txt")])
        self.assertEquals(b.solve(), 1940)

    def test_can_solve_part_2(self):
        b = Bridge([line.strip() for line in open("puzzle_data.txt")])
        self.assertEquals(b.solve_part_2(), (35, 1928))
