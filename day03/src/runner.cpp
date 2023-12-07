#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

#include "day03.hpp"

int main(int argc, char *argv[]) {
    std::ifstream input_file("Day03.txt");

    std::stringstream buffer;
    buffer << input_file.rdbuf();

    std::string input = buffer.str();
    std::cout << "Solve part one: " << solve_part_one(input) << std::endl;
    std::cout << "Solve part two: " << solve_part_two(input) << std::endl;
    return 0;
}
