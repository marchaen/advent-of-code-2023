#include "day03.hpp"

#include <cctype>
#include <vector>

bool is_adjacent_to_symbol(const std::vector<std::string> &lines, int line,
                           int number_start, int number_end) {
    int row_offset = -1;
    int max_rows = 3;

    if (line == 0) {
        row_offset = 0;
        max_rows = 2;
    }

    if (line == lines.size() - 1) {
        max_rows = 2;
    }

    // Note: All input lines need to have the same length. To not cause ub if
    // this assumption is wrong the first access into the line uses the
    // at(int index) method which performs bounds checks.
    int line_length = lines[0].length();

    int column_offset = -1;
    int max_columns = number_end - number_start + 2;

    if (number_end == line_length - 1) {
        max_columns -= 1;
    }

    if (number_start == 0) {
        column_offset = 0;
        max_columns -= 1;
    }

    for (int row = 0; row < max_rows; row++) {
        int actual_row = line + row + row_offset;

        for (int column = 0; column < max_columns; column++) {
            int actual_column = number_start + column + column_offset;

            if (lines[actual_row].at(actual_column) != '.' &&
                ispunct(lines[actual_row][actual_column])) {
                return true;
            }
        }
    }

    return false;
}

int solve_part_one(const std::string &input) {
    using namespace std;

    vector<string> lines;

    // Split on new lines
    int last_line_start = 0;

    for (int index = 0; index < input.length(); index++) {
        if (input[index] == '\n') {
            lines.push_back(
                input.substr(last_line_start, index - last_line_start));
            last_line_start = index + 1;
        }
    }

    // Don't forget the last line
    if (last_line_start != input.length()) {
        lines.push_back(input.substr(last_line_start));
    }

    int part_number = 0;

    for (int line = 0; line < lines.size(); line++) {
        int number_start = -1;

        for (int index = 0; index < lines[line].length(); index++) {
            if (isdigit(lines[line][index])) {
                if (number_start == -1) {
                    number_start = index;
                    continue;
                }
            } else {
                if (number_start == -1) {
                    continue;
                }

                if (is_adjacent_to_symbol(lines, line, number_start, index)) {
                    part_number += stoi(
                        lines[line].substr(number_start, index - number_start));
                }

                number_start = -1;
            }
        }

        // Don't forget the last number
        int number_end = lines[line].length() - 1;

        if (number_start != -1 &&
            is_adjacent_to_symbol(lines, line, number_start, number_end)) {
            part_number += stoi(lines[line].substr(number_start, number_end));
        }
    }

    return part_number;
}

int solve_part_two(const std::string &input) { return 0; }
