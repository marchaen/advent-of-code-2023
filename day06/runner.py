from pathlib import Path

from solutions import part_one, part_two

if __name__ == "__main__":
    input = Path("../input/Day06.txt").read_text()
    print(f"Part One: {part_one(input)}")
    print(f"Part Two: {part_two(input)}")
