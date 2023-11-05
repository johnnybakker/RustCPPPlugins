project(my_devkit_daemon)

set(CMAKE_CXX_STANDARD 17)

add_executable(my_devkit_daemon src/main.cpp)
add_dependencies(my_devkit_daemon my_devkit_plugin)

set(BOOST_INCLUDE_LIBRARIES system filesystem regex dll)
add_subdirectory(deps/boost EXCLUDE_FROM_ALL)

get_target_property(Boost_INCLUDE_DIRS Boost::headers INTERFACE_INCLUDE_DIRECTORIES)
get_target_property(DEVKIT_PLUGIN_DIRS my_devkit_plugin LOCATION)
get_target_property(DEVKIT_PLUGIN_LIB my_devkit_plugin LOCATION_LIB)

include_directories(${Boost_INCLUDE_DIRS} ${DEVKIT_PLUGIN_DIRS})

set(BOOST_LIBRARY_TARGETS Boost::dll Boost::regex Boost::system Boost::filesystem ${Boost_LIBRARIES})
target_link_libraries(my_devkit_daemon ${BOOST_LIBRARY_TARGETS} ${DEVKIT_PLUGIN_LIB})

install(TARGETS my_devkit_daemon RUNTIME DESTINATION bin)