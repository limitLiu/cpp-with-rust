cmake_minimum_required(VERSION 3.15)

set(SHORT_NAME cpp-with-rust)
project(${SHORT_NAME})
add_subdirectory(rs)
add_subdirectory(gui)
