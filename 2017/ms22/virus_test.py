import unittest

from virus import Virus

TESTMAP = ["..#",
           "#..",
           "..."]


class TestVirus(unittest.TestCase):

    def test_can_create_virus_object(self):
        v = Virus()
        self.assertIsNotNone(v)

    def test_can_create_virus_with_map(self):
        v = Virus(TESTMAP)
        self.assertIsNotNone(v)

    def test_can_detect_infection(self):
        v = Virus(TESTMAP)
        self.assertFalse(v.infected())

    def test_can_get_cur_location(self):
        v = Virus(TESTMAP)
        self.assertEquals(v.loc(), (0, 0))

    def test_can_calculate_middle_of_map_for_default(self):
        v = Virus()
        self.assertEquals(v.xoffset, 0)
        self.assertEquals(v.yoffset, 0)

    def test_can_calculate_middle_of_map_for_testdata(self):
        v = Virus(TESTMAP)
        self.assertEquals(v.xoffset, -1)
        self.assertEquals(v.yoffset, -1)

    def test_starts_facing_up(self):
        v = Virus(TESTMAP)
        self.assertEquals(v.direc(), Virus.UP)

    def test_can_turn_right(self):
        v = Virus()
        v.turn_right()
        self.assertEquals(v.direc(), Virus.RIGHT)
        v.turn_right()
        self.assertEquals(v.direc(), Virus.DOWN)
        v.turn_right()
        self.assertEquals(v.direc(), Virus.LEFT)
        v.turn_right()
        self.assertEquals(v.direc(), Virus.UP)

    def test_can_turn_left(self):
        v = Virus()
        v.turn_left()
        self.assertEquals(v.direc(), Virus.LEFT)
        v.turn_left()
        self.assertEquals(v.direc(), Virus.DOWN)
        v.turn_left()
        self.assertEquals(v.direc(), Virus.RIGHT)
        v.turn_left()
        self.assertEquals(v.direc(), Virus.UP)

    def test_can_move_forward(self):
        v = Virus()
        v.move_forward()
        self.assertEquals(v.loc(), (0, -1))

    def test_can_walk_to_infected_location(self):
        v = Virus(TESTMAP)
        v.turn_left()
        v.move_forward()
        self.assertTrue(v.infected())

    def test_can_expand_map_upwards(self):
        v = Virus(TESTMAP)
        y = v.yoffset
        v.expand_upwards()
        self.assertEquals(v.yoffset, y - 1)

    def test_can_expand_map_rightwards(self):
        v = Virus(TESTMAP)
        width = len(v.memmap[0])
        v.expand_rightwards()
        self.assertEquals(len(v.memmap[0]), width + 32)

    def test_can_expand_map_downwards(self):
        v = Virus(TESTMAP)
        height = len(v.memmap)
        v.expand_downwards()
        self.assertEquals(len(v.memmap), height + 1)

    def test_can_remain_in_place_when_expanding_leftwards(self):
        v = Virus(TESTMAP)
        v.turn_left()
        v.move_forward()
        self.assertTrue(v.infected())
        v.expand_leftwards()
        self.assertTrue(v.infected())
        v.move_forward()
        self.assertFalse(v.infected())

    def test_can_autoexpand_up_when_moving(self):
        v = Virus(TESTMAP)
        height = len(v.memmap)
        v.move_forward()
        self.assertEquals(len(v.memmap), height)
        v.move_forward()
        self.assertEquals(len(v.memmap), height + 1)

    def test_can_infect_clean_node(self):
        v = Virus(TESTMAP)
        loc = v.loc()
        self.assertFalse(v.infected())
        v.infect()
        self.assertTrue(v.infected())
        self.assertEquals(v.loc(), loc)

    def test_can_clean_infected_node(self):
        v = Virus(TESTMAP)
        v.turn_left()
        v.move_forward()
        self.assertTrue(v.infected())
        v.clean()
        self.assertFalse(v.infected())

    def test_can_take_first_step(self):
        v = Virus(TESTMAP)
        v.burst()
        self.assertEquals(v.direc(), Virus.LEFT)
        self.assertEquals(v.loc(), (-1, 0))
        self.assertEquals(v.infections, 1)

    def test_can_take_several_steps(self):
        v = Virus(TESTMAP)
        v.burst()
        self.assertTrue(v.infected())
        v.burst()
        self.assertTrue(v.direc(), Virus.UP)
        self.assertEquals(v.infections, 1)
        self.assertFalse(v.infected())
        v.burst()
        v.burst()
        v.burst()
        v.burst()
        self.assertEquals(v.infections, 5)
        self.assertEquals(v.direc(), Virus.UP)
        v.burst()
        self.assertEquals(v.infections, 5)

    def test_can_solve_example_problem(self):
        v = Virus(TESTMAP)
        for i in range(70):
            v.burst()
        self.assertEquals(v.infections, 41)

    def test_can_work_multiple_infection_stages(self):
        v = Virus(TESTMAP)
        self.assertFalse(v.infected())
        v.next_infection_stage()
        self.assertTrue(v.weakened())

        self.assertEquals(v.infections, 0)
        v.next_infection_stage()
        self.assertTrue(v.infected())
        self.assertEquals(v.infections, 1)

        v.next_infection_stage()
        self.assertTrue(v.flagged())

    def test_can_solve_evolved_example(self):
        v = Virus(TESTMAP)
        for i in range(100):
            v.evolved_burst()
        self.assertEquals(v.infections, 26)

    def test_can_solve_extended_evolved_example(self):
        v = Virus(TESTMAP)
        for i in range(10000000):
            v.evolved_burst()
        self.assertEquals(v.infections, 2511944)

    def test_can_solve_part_1(self):
        v = Virus([line.strip() for line in open("puzzle_data.txt")])
        for i in range(10000):
            v.burst()
        self.assertEquals(v.infections, 5196)

    # def test_can_solve_part_2(self):
    #     v = Virus([line.strip() for line in open("puzzle_data.txt")])
    #     for i in range(10000000):
    #         v.evolved_burst()
    #     self.assertEquals(v.infections, 2511633)
