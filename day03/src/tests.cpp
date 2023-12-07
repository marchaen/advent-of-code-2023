#include <iostream>

#include "day03.hpp"

void test_part_one(const std::string &input) {
    const auto expected = 4361;
    auto result = solve_part_one(input);

    std::cout << "Testing part one: ";

    if (result == expected) {
        std::cout << "Sucess" << std::endl;
    } else {
        std::cout << "Failure" << std::endl;
        std::cout << "Expected: " << expected << " Got: " << result
                  << std::endl;
    }
}

int main(int argc, char *argv[]) {
    auto input = R"(467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..)";

    test_part_one(input);
    return 0;
}