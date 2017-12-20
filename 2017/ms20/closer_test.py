import math
import unittest

from closer import Closer, Particle

TESTDATA = [
    "p=<5528,2008,1661>, v=<-99,-78,-62>, a=<-17,-2,-2>",
    "p=<3088,2748,-1039>, v=<-103,-136,94>, a=<-5,0,-4>",
    "p=<628,-1052,161>, v=<20,33,-92>, a=<-5,2,8>",
]


class CloserTest(unittest.TestCase):

    def test_can_create_a_closer(self):
        c = Closer()
        self.assertIsNotNone(c)

    def test_can_parse_input(self):
        c = Closer(TESTDATA)
        self.assertEqual(c.particles[0].pos, [5528, 2008, 1661])
        self.assertEqual(c.particles[1].vel, [-103, -136, 94])
        self.assertEqual(c.particles[2].acc, [-5, 2, 8])

    def test_can_move_one_step(self):
        c = Closer(["p=<0, 1, 2>, v=<1, 2, 3>, a=<2, 3, 4>"])
        c.step()
        self.assertEqual(c.particles[0].vel, [3, 5, 7])
        self.assertEqual(c.particles[0].pos, [3, 6, 9])

    def test_can_find_distance(self):
        c = Closer([
            "3,4,0,1,1,1,1,1,1",
            "1,2,3,2,2,2,2,2,2",
        ])
        self.assertEqual(c.distance(c.particles[0].pos), 7)
        self.assertEqual(c.distance(c.particles[1].pos), 6)

    def test_can_find_closest(self):
        c = Closer(TESTDATA)
        self.assertEqual(c.closest(), 2)

    def test_can_find_direction_away_with_constant_velocity(self):
        c = Closer([
            "0,0,0,1,0,0,0,0,0",
        ])
        self.assertEqual(c.direction()[0], Closer.AWAY)

    def test_can_find_direction_towards_with_constant_velocity(self):
        c = Closer([
            "10,0,0,-1,0,0,0,0,0"
        ])
        self.assertEqual(c.direction()[0], Closer.TOWARDS)

    def test_response_is_towards_but_acc_is_away(self):
        c = Closer(["5,5,5,-3,0,0,1,0,0"])
        self.assertEqual(c.direction()[0], Closer.TOWARDS)

    def test_can_detect_slowing_down(self):
        c = Closer(["5,5,5,3,0,0,-1,0,0"])
        self.assertEqual(c.delta_v()[0], Closer.SLOWING_DOWN)

    def test_can_detect_speeding_up(self):
        c = Closer(["5,5,5,0,3,0,0,0,1"])
        self.assertEqual(c.delta_v()[0], Closer.SPEEDING_UP)

    def test_can_find_smallest_acceleration(self):
        c = Closer(TESTDATA)
        self.assertEqual(c.smallest_acc(), 1)

    def test_can_solve_part_1(self):
        plist = [line.strip() for line in open("puzzle_data.txt")]
        c = Closer(plist)
        self.assertEqual(c.smallest_acc(), 364)

    def test_can_collide_particles(self):
        c = Closer(["1,1,1,0,0,0,0,0,0", "1,1,1,1,1,1,1,1,1"])
        self.assertTrue(c.particles[0].collides_with(c.particles[1]))

    def test_can_detect_collisions(self):
        c = Closer([
            "1,1,1,0,0,0,0,0,0",
            "1,2,1,0,0,0,0,0,0",
            "1,1,1,1,1,1,1,1,1"
        ])
        self.assertEqual(len(c.particles), 3)
        c.remove_collisions()
        self.assertEqual(len(c.particles), 1)

    def test_can_detect_collisions_after_one_step(self):
        c = Closer([
            "3,3,3,0,0,0,0,0,0",
            "1,2,1,0,0,0,0,0,0",
            "1,1,1,1,1,1,1,1,1"
        ])
        c.remove_collisions()
        self.assertEqual(len(c.particles), 3)
        c.step()
        c.remove_collisions()
        self.assertEqual(len(c.particles), 1)

    def test_can_solve_part_2(self):
        plist = [line.strip() for line in open("puzzle_data.txt")]
        c = Closer(plist)
        c.remove_collisions()
        for i in range(50):
            c.step()
            c.remove_collisions()
        self.assertEqual(len(c.particles), 420)

    def test_can_collide_four_from_example(self):
        c = Closer([
            "p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>",
            "p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>",
            "p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>",
            "p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>",
        ])
        self.assertEqual(len(c.particles), 4)
        c.step()
        c.remove_collisions()
        self.assertEqual(len(c.particles), 4)
        c.step()
        c.remove_collisions()
        self.assertEqual(len(c.particles), 1)
