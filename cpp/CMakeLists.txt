set(CMAKE_CXX_STANDARD 14)


add_executable(main src/main.cpp)
add_dependencies(main rust_plugin_manager)

get_target_property(RUST_PLUGIN_DIR rust_plugin_manager LOCATION)

include_directories(${RUST_PLUGIN_DIR})
target_link_libraries(main ${RUST_PLUGIN_DIR}/rust_plugin_manager.lib)

install(TARGETS main RUNTIME DESTINATION bin)