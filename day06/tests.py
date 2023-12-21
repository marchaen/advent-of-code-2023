import unittest

from solutions import part_one


class Tests(unittest.TestCase):
    def test_part_one(self):
        input = """Time:      7  15   30
Distance:  9  40  200"""

        self.assertEqual(part_one(input), 288)


if __name__ == "__main__":
    unittest.main()
