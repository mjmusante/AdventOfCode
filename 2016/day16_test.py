import unittest

from day16 import DCurve

class DCurveTest(unittest.TestCase):

    def test_one_bit_works(self):
        b = DCurve("1")
        self.assertEqual(b.result(), "100")
        b = DCurve("0")
        self.assertEqual(b.result(), "001")

    def test_five_bits_works(self):
        b = DCurve("11111")
        self.assertEqual(b.result(), "11111000000")

    def test_lotsa_bits_works(self):
        b = DCurve("111100001010")
        self.assertEqual(b.result(), "1111000010100101011110000")

    def test_cksum_calc_for_odd(self):
        self.assertEqual(DCurve.calc_cksum("0"), False)

    def test_cksum_calc_for_even(self):
        self.assertEqual(DCurve.calc_cksum("110010110100"), "100")

    def test_full_checksum(self):
        b = DCurve("10000")
        self.assertEqual(b.checksum(20), "01100")