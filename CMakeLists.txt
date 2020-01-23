cmake_minimum_required (VERSION 3.1...3.15)
project (SudokuGenerator VERSION 0.9 DESCRIPTION "Functions to implement a sudoku generator")

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)


set (CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -fsanitize=address")
set (CMAKE_LINKER_FLAGS_DEBUG "${CMAKE_LINKER_FLAGS_DEBUG} -fsanitize=address")
set(default_build_type "Debug")

add_executable(SudokuGenerator sudoko_functions.cpp)
