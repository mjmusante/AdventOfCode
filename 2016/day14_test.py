import unittest

from day14 import OneTimePad

class day14test(unittest.TestCase):

    def test_first_triple(self):
        otp = OneTimePad("abc")
        self.assertEqual(otp.triple(0), 18)

    def test_18_is_not_a_key(self):
        otp = OneTimePad("abc")
        self.assertEqual(otp.is_key(18), False)

    def test_tuples(self):
        rslts = [(3, '2')]
        i = OneTimePad.find_tuples("011222333344444", 3)[0]
        self.assertIn(i, rslts)
        rslts.remove(i)
        self.assertEqual(len(rslts), 0)

    def test_39_is_a_key(self):
        otp = OneTimePad("abc")
        self.assertEqual(otp.is_key(39), True)

    def test_40_through_100(self):
        otp = OneTimePad("abc")
        for i in range(40, 101):
            self.assertEqual(otp.is_key(i), i == 92)

    def test_stretching(self):
        otp = OneTimePad("abc")
        for i in range(0, 20):
            self.assertEqual(otp.is_key(i, True), i == 10)