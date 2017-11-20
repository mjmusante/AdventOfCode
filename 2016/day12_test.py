import unittest

from day12 import AssemBunny

class TestCanCreateComputer(unittest.TestCase):
    def test_create(self):
        assem = AssemBunny("inc a")
        self.assertEqual(assem.ip, 0)

    def test_splits_multiple_instructions(self):
        assem = AssemBunny("inc a; inc b")
        self.assertEqual(len(assem.inst), 2)

    def test_can_increment_register(self):
        assem = AssemBunny("inc a; inc b")
        assem.run()
        self.assertEqual(assem.reg['a'], 1)
        self.assertEqual(assem.reg['b'], 1)

    def test_copy_number_into_register(self):
        assem = AssemBunny("cpy 42 c")
        assem.run()
        self.assertEqual(assem.reg['c'], 42)

    def test_copy_reg_into_reg(self):
        assem = AssemBunny("inc a; cpy a d")
        assem.run()
        self.assertEqual(assem.reg['d'], 1)

    def test_decrement_register(self):
        assem = AssemBunny("inc a; dec a")
        assem.run()
        self.assertEqual(assem.reg['a'], 0)

    def test_negative_copy(self):
        assem = AssemBunny("cpy -5 a")
        assem.run()
        self.assertEqual(assem.reg['a'], -5)

    def test_jnz_on_register(self):
        assem = AssemBunny("cpy 3 a; dec a; jnz a -1")
        assem.run();
        self.assertEqual(assem.reg['a'], 0)

    def test_jnz_on_number(self):
        assem = AssemBunny("jnz 1 2; inc a")
        assem.run()
        self.assertEqual(assem.reg['a'], 0)

    def test_sample_input(self):
        assem = AssemBunny("cpy 41 a;inc a;inc a;dec a;jnz a 2;dec a")
        assem.run()
        self.assertEqual(assem.reg['a'], 42)

    def test_exception_on_bad_instruction(self):
        assem = AssemBunny("cmp a b")
        with self.assertRaises(Exception):
            assem.run()
