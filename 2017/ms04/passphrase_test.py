import unittest

from passphrase import count_valid

class TestPassphrase(unittest.TestCase):
    def test_validity(self):
        lines = ["aa bb cc dd ee", "aa bb cc dd aa", "aa bb cc dd aaa"]
        self.assertEquals(count_valid(lines), 2)

    def test_anagram_validity(self):
        lines = [
            "abcde fghij",
            "abcde xyz ecdab",
            "a ab abc abd abf abj",
            "iiii oiii ooii oooi oooo",
            "oiii ioii iioi iiio"]
        self.assertEquals(count_valid(lines, True), 3)