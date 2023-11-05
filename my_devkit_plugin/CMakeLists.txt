project(my_devkit_plugin)

cargo_library(${PROJECT_NAME})

get_target_property(INSTALL_DLL ${PROJECT_NAME} LOCATION_DLL)	
install(FILES ${INSTALL_DLL} DESTINATION bin)