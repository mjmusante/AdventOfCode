import unittest

from captcha import Captcha


class TestCaptcha(unittest.TestCase):

    def test_part_1(self):
        c = Captcha("1122")
        self.assertEqual(c.sumMatch(), 3)

        c = Captcha("1111")
        self.assertEqual(c.sumMatch(), 4)

        c = Captcha("1234")
        self.assertEqual(c.sumMatch(), 0)

        c = Captcha("91212129")
        self.assertEqual(c.sumMatch(), 9)


    def test_part_2(self):
        c = Captcha("1212")
        self.assertEqual(c.sumHalfway(), 6)

        c = Captcha("1221")
        self.assertEqual(c.sumHalfway(), 0)

        c = Captcha("123425")
        self.assertEqual(c.sumHalfway(), 4)

        c = Captcha("123123")
        self.assertEqual(c.sumHalfway(), 12)

        c = Captcha("12131415")
        self.assertEqual(c.sumHalfway(), 4)