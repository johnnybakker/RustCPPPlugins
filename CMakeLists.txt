cmake_minimum_required(VERSION 3.28.0)
project(CMakeRust)

enable_testing()
add_subdirectory(rust_plugin_manager)
add_subdirectory(my_plugin)
add_subdirectory(cpp)