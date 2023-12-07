cmake_minimum_required(VERSION 3.0)
set(CMAKE_CXX_STANDARD 20)

project(day03)

configure_file(../input/Day03.txt Day03.txt COPYONLY)
add_library(day03 STATIC src/day03.cpp)

add_executable(runner src/runner.cpp)
target_link_libraries(runner day03)

add_executable(tests src/tests.cpp)
target_link_libraries(tests day03)

