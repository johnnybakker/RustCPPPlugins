cmake_minimum_required(VERSION 3.5)
project(CMakeRust)

enable_testing()

function(cargo_library target_name)

	set(COMPILE_MESSAGE "Compiling ${target_name}")
	set(CARGO_BUILD_CMD cargo build)
	set(CARGO_TEST_CMD cargo test)
	set(TARGET_DIR "debug")

	if (CMAKE_BUILD_TYPE STREQUAL "Release")
		set(COMPILE_MESSAGE "${COMPILE_MESSAGE} in release mode")
		set(CARGO_BUILD_CMD ${CARGO_BUILD_CMD} --release)
		set(CARGO_TEST_CMD ${CARGO_BUILD_CMD} --release)
		set(TARGET_DIR "release")
	endif()

	set(TARGET_DLL ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/${target_name}.dll)
	set(OUTPUT_DLL ${CMAKE_CURRENT_BINARY_DIR}/lib${target_name}.dll)

	set(TARGET_LIB ${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/${target_name}.dll.lib)
	set(OUTPUT_LIB ${CMAKE_CURRENT_BINARY_DIR}/lib${target_name}.lib)

	set(BUILD_CMD ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_BUILD_CMD})
	set(TEST_CMD ${CMAKE_COMMAND} -E env CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_TEST_CMD})

	set(COPY_DLL_CMD ${CMAKE_COMMAND} -E copy_if_different ${TARGET_DLL} ${OUTPUT_DLL})
	set(COPY_LIB_CMD ${CMAKE_COMMAND} -E copy_if_different ${TARGET_LIB} ${OUTPUT_LIB})

	add_custom_target(${PROJECT_NAME} ALL
		COMMENT ${COMPILE_MESSAGE}
		COMMAND ${BUILD_CMD}
		COMMAND ${COPY_DLL_CMD}
		COMMAND ${COPY_LIB_CMD}
		WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
	)

    set_target_properties(${target_name} PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})
    set_target_properties(${target_name} PROPERTIES	LOCATION_DLL ${OUTPUT_DLL})
    set_target_properties(${target_name} PROPERTIES LOCATION_LIB ${OUTPUT_LIB})

	add_test(NAME ${PROJECT_NAME}_test COMMAND ${CARGO_TEST_CMD} WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
	
endfunction()

add_subdirectory(my_devkit_plugin)
add_subdirectory(my_devkit_plugins)
add_subdirectory(my_devkit_daemon)