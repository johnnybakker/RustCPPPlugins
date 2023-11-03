
if (CMAKE_BUILD_TYPE STREQUAL "Release")
	set(CARGO_CMD cargo build --release)
	set(TARGET_DIR "release")
else()
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
endif ()

add_custom_target(my_plugin ALL
    COMMAND ${CMAKE_COMMAND} -E env "CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR}" ${CARGO_CMD} --target-dir ${CMAKE_CURRENT_BINARY_DIR}
	COMMAND ${CMAKE_COMMAND} -E copy_if_different "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/my_plugin.dll.lib" "${CMAKE_CURRENT_BINARY_DIR}/my_plugin.lib"
	COMMAND ${CMAKE_COMMAND} -E copy_if_different "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/my_plugin.dll" "${CMAKE_CURRENT_BINARY_DIR}/my_plugin.dll"
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

set_target_properties(my_plugin PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_test(
	NAME my_plugin_test 
	COMMAND cargo test 
	WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)

install(FILES "${CMAKE_CURRENT_BINARY_DIR}/my_plugin.dll" DESTINATION bin/plugins)