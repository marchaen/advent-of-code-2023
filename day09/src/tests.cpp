#include <iostream>

#include "day09.hpp"

void test_part_one(const std::string &input) {
    const auto expected = 114;
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
    auto input = R"(0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45)";

    test_part_one(input);
    return 0;
}
