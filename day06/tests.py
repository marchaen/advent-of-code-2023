import unittest

from solutions import part_one, part_two

input = """Time:      7  15   30
Distance:  9  40  200"""


class Tests(unittest.TestCase):
    def test_part_one(self):
        self.assertEqual(part_one(input), 288)

    def test_part_two(self):
        self.assertEqual(part_two(input), 71503)


if __name__ == "__main__":
    unittest.main()
