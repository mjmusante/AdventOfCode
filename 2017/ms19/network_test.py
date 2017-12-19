import unittest

from network import Diagram

LINES = [
    "     |          ",
    "     |  +--+    ",
    "     A  |  C    ",
    " F---|----E|--+ ",
    "     |  |  |  D ",
    "     +B-+  +--+ ",
]


class TestNetwork(unittest.TestCase):

    def test_can_create_empty_diagram(self):
        d = Diagram()
        self.assertIsNotNone(d)

    def test_can_create_example_diagram(self):
        d = Diagram(LINES)
        self.assertIsNotNone(d)

    def test_can_find_starting_point(self):
        d = Diagram(LINES)
        self.assertEqual(d.start(), (5, 0, Diagram.DOWN))

    def test_check_left_right_to_line(self):
        d = Diagram(LINES)
        self.assertEqual(d.check_left_right(8, 1), Diagram.RIGHT)

    def test_check_up_down_to_line(self):
        d = Diagram(LINES)
        self.assertEqual(d.check_up_down(8, 1), Diagram.DOWN)

    def test_check_left_for_letter(self):
        d = Diagram(LINES)
        self.assertEqual(d.check_left_right(5, 5), Diagram.RIGHT)

    def test_one_step_moves_down(self):
        d = Diagram(LINES)
        pos = d.start()
        pos = d.moveFrom(pos)
        self.assertEqual(pos[0], 5)
        self.assertEqual(pos[1], 1)
        self.assertEqual(pos[2], Diagram.DOWN)

    def test_can_turn_on_its_own(self):
        d = Diagram(LINES)
        pos = d.start()
        for i in range(5):
            pos = d.moveFrom(pos)
        self.assertEqual(pos[0], 5)
        self.assertEqual(pos[1], 5)
        self.assertEqual(pos[2], Diagram.RIGHT)

    def test_can_halt(self):
        d = Diagram(LINES)
        pos = d.start()
        while pos[2] != Diagram.HALT:
            pos = d.moveFrom(pos)
        self.assertEqual("".join(d.seq), "ABCDEF")

    def test_can_count_steps(self):
        d = Diagram(LINES)
        pos = d.start()
        while pos[2] != Diagram.HALT:
            pos = d.moveFrom(pos)
        self.assertEqual(d.steps, 38)

    def test_can_pass_part_1_and_part_2(self):
        puzzle = [line for line in open("puzzle_data.txt")]
        d = Diagram(puzzle)
        pos = d.start()
        while pos[2] != Diagram.HALT:
            # print("(%s, %s, %s)" % pos)
            pos = d.moveFrom(pos)
        self.assertEqual("".join(d.seq), "RYLONKEWB")
        self.assertEqual(d.steps, 16016)
