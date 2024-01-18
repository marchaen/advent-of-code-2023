#include <cstdlib>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

#include "day09.hpp"

int main(int argc, char *argv[]) {
    std::ifstream input_file("Input.txt");

    if (!input_file.good()) {
        std::cout << "The \"Input.txt\" file couldn't be found in the cwd.";
        std::exit(1);
    }

    std::stringstream buffer;
    buffer << input_file.rdbuf();

    std::string input = buffer.str();
    std::cout << "Solve part one: " << solve_part_one(input) << std::endl;
    return 0;
}
