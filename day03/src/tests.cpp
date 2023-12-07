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

void test_part_two(const std::string &input) {
    const auto expected = 467835;
    auto result = solve_part_two(input);

    std::cout << "Testing part two: ";

    if (result == expected) {
        std::cout << "Sucess" << std::endl;
    } else {
        std::cout << "Failure" << std::endl;
        std::cout << "Expected: " << expected << " Got: " << result
                  << std::endl;
    }
}

void better_test_part_two() {
    auto input = R"(12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90$12...
............
2.2......12.
.$.........*
1.1..503+.56)";
    auto expected = 6756;
    auto result = solve_part_two(input);

    std::cout << "Testing part two other: ";

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
    test_part_two(input);
    better_test_part_two();
    return 0;
}
