cmake_minimum_required(VERSION 3.22)
set(CMAKE_CXX_STANDARD 23)

project(advent-of-code-2023)

set(ENV{DAY_DIGITS} "03")
add_subdirectory(day$ENV{DAY_DIGITS})

set(ENV{DAY_DIGITS} "09")
add_subdirectory(day$ENV{DAY_DIGITS})

