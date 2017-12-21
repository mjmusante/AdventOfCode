import unittest

from rotor import Rotor, Pattern, Grid


class TestRotor(unittest.TestCase):

    def test_can_create_3x3_rotor(self):
        r = Rotor()
        self.assertIsNotNone(r)

    def test_can_generate_initial_state(self):
        r = Rotor()
        self.assertEquals(r.state(), ".#...####")

    def test_can_rotate_3x3_rotor(self):
        r = Rotor()
        q = r.rotate()
        self.assertEquals(q.state(), ".###.#..#")

    def test_can_flip_3x3_rotro(self):
        r = Rotor()
        self.assertEquals(r.flip().state(), ".#.#..###")

    def test_can_create_2x2_rotor(self):
        r = Rotor("#...")
        self.assertEquals(r.state(), "#...")

    def test_can_rotate_2x2_rotor(self):
        r = Rotor("#...")
        self.assertEquals(r.rotate().state(), "..#.")

    def test_can_flip_2x2_rotor(self):
        r = Rotor("#...")
        self.assertEquals(r.flip().state(), ".#..")

    def test_can_process_2x2_pattern(self):
        p = Pattern()
        p.add_pattern("#./.. => .../#../#..")
        self.assertEquals(len(p.plist), 4)

    def test_can_process_3x3_pattern(self):
        p = Pattern()
        p.add_pattern("###/#../... => ..../#.../#..#/#..#")
        self.assertEquals(len(p.plist), 8)

    def test_can_apply_pattern_to_3x3(self):
        p = Pattern()
        p.add_pattern(".#./..#/### => #..#/..../..../#..#")
        r = Rotor(".#...####")
        s = p.apply_to(r)
        self.assertEquals(s.state(), "#..#........#..#")

    def test_can_apply_pattern_to_2x2(self):
        p = Pattern()
        p.add_pattern("../.# => ##./#../...")
        (w, x, y, z) = (Rotor("#..."), Rotor(".#.."),
                        Rotor("..#."), Rotor("...#"))
        for i in [w, x, y, z]:
            self.assertEquals(p.apply_to(i).state(), "##.#.....")

    def test_can_split_4x4_rotor(self):
        r = Rotor("##....####....##")
        ary = r.split()
        self.assertEquals(ary[0][0].state(), "##..")
        self.assertEquals(ary[0][1].state(), "..##")
        self.assertEquals(ary[1][0].state(), "##..")
        self.assertEquals(ary[1][1].state(), "..##")

    def test_can_create_grid(self):
        g = Grid(".#...####", None)
        self.assertIsNotNone(g)

    def test_can_split_4x4_into_four_2x2s(self):
        g = Grid("#..#........#..#", None)
        g.fracture()
        self.assertEquals(g.grid[0][0].state(), "#...")
        self.assertEquals(g.grid[0][1].state(), ".#..")
        self.assertEquals(g.grid[1][0].state(), "..#.")
        self.assertEquals(g.grid[1][1].state(), "...#")

    def test_can_split_6x6_into_nine_2x2s(self):
        r = Rotor("##.##.#..#........##.##.#..#........").split()
        self.assertEquals(r[0][0].state(), "###.")
        self.assertEquals(r[1][1].state(), "...#")
        self.assertEquals(r[2][2].state(), "....")

    def test_can_split_9x9_into_three_3x3s(self):
        layout = \
            "##.##.##." \
            "........." \
            ".....#.#." \
            "##.##.##." \
            "........." \
            ".###..#.#" \
            "##.##.##." \
            "........#" \
            "##.###..."
        r = Rotor(layout).split()
        self.assertEquals(r[0][0].state(), "##.......")
        self.assertEquals(r[1][1].state(), "##....#..")
        self.assertEquals(r[2][2].state(), "##...#...")

    def test_grid_can_apply_pattern_to_3x3(self):
        p = Pattern()
        p.add_pattern(".#./..#/### => #..#/..../..../#..#")
        g = Grid(".#...####", p)
        g.do_patterns()
        self.assertEquals(g.grid[0][0].state(), "#..#........#..#")

    def test_grid_can_rejoin_6x6_after_fracture(self):
        layout = "##.##.#..#........##.##.#..#........"
        g = Grid(layout, None)
        g.fracture()
        g.rejoin()
        self.assertEquals(g.grid[0][0].state(), layout)

    def test_grid_can_rejoin_4x4_after_applying_3x3_pattern(self):
        p = Pattern()
        p.add_pattern("#./.. => ##./#../...")
        p.add_pattern("##./#../... => ..../.##./.##./....")
        g = Grid("#..#........#..#", p)
        g.fracture()        # 4x4 -> 2x2 of 2x2
        g.do_patterns()     # 2x2 of 2x2 -> 2x2 of 3x3
        g.do_patterns()     # 2x2 of 3x3 -> 2x2 of 4x4
        g.rejoin()          # 2x2 of 4x4 -> 8x8
        self.assertEquals(len(g.grid), 1)

    def test_grid_can_handle_multple_patterns(self):
        p = Pattern()
        p.add_pattern("../.# => ##./#../...")
        p.add_pattern(".#./..#/### => #..#/..../..../#..#")
        g = Grid(".#...####", p)
        g.one_move()
        self.assertEquals(g.grid[0][0].state(), "#..#........#..#")
        g.one_move()
        self.assertEquals(g.grid[0][0].state(), '##.#.....')
        self.assertEquals(g.grid[0][1].state(), '##.#.....')
        self.assertEquals(g.grid[1][0].state(), '##.#.....')
        self.assertEquals(g.grid[1][1].state(), '##.#.....')

    def test_grid_can_count_pixels(self):
        p = Pattern()
        p.add_pattern("../.# => ##./#../...")
        p.add_pattern(".#./..#/### => #..#/..../..../#..#")
        g = Grid(".#...####", p)

        g.rejoin()
        g.fracture()
        g.do_patterns()     # 3x3 -> 4x4

        g.rejoin()
        g.fracture()        # 4x4 -> 2x2 of 2x2
        g.do_patterns()     # 2x2 of 2x2 -> 2x2 of 3x3
        self.assertEquals(g.grid[0][0].state(), "##.#.....")
        self.assertEquals(g.grid[1][1].state(), "##.#.....")
        self.assertEquals(g.count_pixels(), 12)

        g.rejoin()          # 2x2 of 3x3 -> 6x6
        self.assertEquals(g.count_pixels(), 12)

        g.fracture()        # 6x6 -> 3x3 of 2x2
        self.assertEquals(g.count_pixels(), 12)

    def test_grid_can_solve_part_1_and_part_2(self):
        p = Pattern()
        lines = [line.strip() for line in open("puzzle_data.txt")]
        for l in lines:
            p.add_pattern(l)

        g = Grid(".#...####", p)
        for i in range(5):
            g.one_move()

        self.assertEquals(g.count_pixels(), 147)

        for i in range(13):
            g.rejoin()
            g.fracture()
            g.do_patterns()

        self.assertEquals(g.count_pixels(), 1936582)
