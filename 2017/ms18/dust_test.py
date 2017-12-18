import unittest

from dust import Chip, IllegalInstructionException

class DustTest(unittest.TestCase):

    def test_can_create_chip(self):
        c = Chip()
        self.assertEqual(c.pc, 0)

    def test_can_set_register_to_value(self):
        c = Chip()
        self.assertEqual(c.reg['a'], 0)
        c.ex("set a 42")
        self.assertEqual(c.reg['a'], 42)

    def test_can_set_register_to_another_register(self):
        c = Chip()
        c.ex("set a 43")
        c.ex("set b a")
        self.assertEqual(c.reg['b'], 43)

    def test_can_set_reg_to_negative_number(self):
        c = Chip()
        c.ex("set c -3")
        self.assertEqual(c.reg['c'], -3)

    def test_can_set_snd_command_with_value(self):
        c = Chip()
        c.ex("snd 44")
        self.assertEqual(c.snd, 44)

    def test_can_set_snd_with_register(self):
        c = Chip()
        c.ex("set d 45")
        c.ex("snd d")
        self.assertEqual(c.snd, 45)

    def test_can_rcv_value_from_snd(self):
        c = Chip()
        c.ex("snd 46")
        c.ex("set e 1")
        c.ex("rcv e")
        self.assertEqual(c.reg['e'], 46)

    def test_no_rcv_if_0(self):
        c = Chip()
        c.ex("snd 46")
        c.ex("rcv e")
        self.assertEqual(c.reg['e'], 0)


    def test_can_add_immediate_value(self):
        c = Chip()
        c.ex("add f 47")
        self.assertEqual(c.reg['f'], 47)

    def test_can_add_regs(self):
        c = Chip()
        c.ex("set g 18")
        c.ex("set h 30")
        c.ex("add h g")
        self.assertEqual(c.reg['h'], 18 + 30)

    def test_can_subtract_regs(self):
        c = Chip()
        c.ex("set c 10")
        c.ex("add c -1")
        self.assertEqual(c.reg['c'], 10 - 1)

    def test_can_mul_regs(self):
        c = Chip()
        c.ex("set i 3")
        c.ex("set j 5")
        c.ex("mul i j")
        self.assertEqual(c.reg['i'], 3 * 5)

    def test_can_mod_regs(self):
        c = Chip()
        c.ex("set k 12")
        c.ex("set l 7")
        c.ex("mod k l")
        self.assertEqual(c.reg['k'], 12 % 7)

    def test_pc_starts_at_0(self):
        c = Chip()
        self.assertEqual(c.pc, 0)

    def test_pc_moves_forward(self):
        c = Chip()
        c.ex("set a 1")
        self.assertEqual(c.pc, 1)

    def test_pc_jumps_gz(self):
        c = Chip()
        c.ex("set a 1")
        c.ex("jgz a -1")
        self.assertEqual(c.pc, 0)

    def test_pc_jumps_gz_with_register(self):
        c = Chip()
        c.ex("set a 2")
        c.ex("jgz a a")
        self.assertEqual(c.pc, 3)

    def test_pc_jumps_backwards(self):
        c = Chip()
        c.ex("set a 1")
        c.ex("jgz a -12")
        self.assertEqual(c.pc, -11)

    def test_pc_does_not_jump_if_0(self):
        c = Chip()
        c.ex("jgz a -1")
        self.assertEqual(c.pc, 1)

    def test_rejects_garbage_commands(self):
        c = Chip()
        with self.assertRaises(IllegalInstructionException):
            c.ex("set a 2jkl")

    def test_can_run_small_prg(self):
        prg = [
            "set a 1",
            "set b 2",
            "set c 10",
            "add c -1",
            "mul a b",
            "jgz c -2"
        ]
        c = Chip()
        c.run_loopback(prg)
        self.assertEqual(c.reg['a'], 1024)

    def test_simple_halt_on_rcv(self):
        prg = [
            "snd 42",
            "set a 1",
            "rcv a",
            "set a 88"
        ]
        c = Chip()
        c.run_loopback(prg)
        self.assertEqual(c.reg['a'], 42)

    def test_can_pass_part_1(self):
        prg = [line.strip() for line in open("puzzle_data.txt")]
        c = Chip()
        c.run_loopback(prg)
        self.assertEqual(c.snd, 8600)

    def test_can_send_multiple_values(self):
        c = Chip()
        c.ex("snd 1")
        c.ex("snd 2")
        self.assertEqual(c.queue, [1, 2])

    def test_can_get_next_entry_from_queue(self):
        c = Chip()
        c.ex("snd 3")
        self.assertEqual(c.qpop(), 3)

    def test_can_pop_multiple_from_queue(self):
        c = Chip()
        c.ex("snd 4")
        c.ex("snd 5")
        c.qpop()
        self.assertEqual(c.qpop(), 5)

    def test_can_rcv_value_from_another_source(self):
        c = Chip()
        c.run(["rcv a"])
        c.cont(["rcv a"], 12)
        self.assertEqual(c.reg['a'], 12)

    def test_can_check_empty_queues(self):
        c = Chip()
        self.assertFalse(c.has_values())

    def test_can_check_full_queues(self):
        c = Chip()
        c.ex("snd 1")
        self.assertTrue(c.has_values())

    def test_can_see_if_blocked(self):
        c = Chip()
        c.ex("rcv a")
        self.assertTrue(c.is_blocked())

    def test_can_jgz_immediate_value(self):
        c = Chip()
        c.ex("jgz 1 0")
        self.assertEqual(c.pc, 0)

    def test_no_values_set_at_start(self):
        c = Chip()
        self.assertEqual(c.sent_count, 0)


    def test_can_count_values_sent(self):
        c = Chip()
        for i in range(10):
            c.ex("snd %s" % i)
        self.assertEqual(c.sent_count, 10)

    def test_can_count_even_after_popping(self):
        c = Chip()
        for i in range(10):
            c.ex("snd %s" % i)
            self.assertEqual(c.qpop(), i)
        self.assertEqual(c.sent_count, 10)

    def test_can_start_chip_with_default_id(self):
        c = Chip()
        self.assertEqual(c.reg['p'], 0)

    def test_can_start_chip_with_another_id(self):
        c = Chip(42)
        self.assertEqual(c.reg['p'], 42)

    def dual_run_prg(self, prg):
        c1 = Chip(0)
        c2 = Chip(1)

        c1.run(prg)
        c2.run(prg)

        # at this point, they're both blocked on rcv
        while True:
            while c1.has_values() and c2.is_blocked():
                val = c1.qpop()
                c2.cont(prg, val)

            while c2.has_values() and c1.is_blocked():
                val = c2.qpop()
                c1.cont(prg, val)

            if (c1.is_blocked() and not c2.has_values()) \
                and (c2.is_blocked() and not c1.has_values()):
                    break

            if not c1.is_blocked() and not c2.is_blocked():
                break

        return (c1, c2)


    def test_blockage(self):
        prg = ["snd 1", "snd 2", "snd p", "rcv a", "rcv b", "rcv c", "rcv d"]
        (p0, p1) = self.dual_run_prg(prg)

        self.assertEqual(p0.reg['a'], 1)
        self.assertEqual(p0.reg['b'], 2)
        self.assertEqual(p0.reg['c'], 1)

        self.assertEqual(p1.reg['a'], 1)
        self.assertEqual(p1.reg['b'], 2)
        self.assertEqual(p1.reg['c'], 0)

        self.assertEqual(p0.sent_count, 3)
        self.assertEqual(p1.sent_count, 3)

    def test_can_pass_part_2(self):
        prg = [line.strip() for line in open("puzzle_data.txt")]
        (p0, p1) = self.dual_run_prg(prg)
        self.assertEqual(p1.sent_count, 7239)
