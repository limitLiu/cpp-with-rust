set(CMAKE_CXX_STANDARD 14)

include(FindSDL2.cmake)
find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})
include_directories("${PROJECT_SOURCE_DIR}/includes")

set(source_dir "${PROJECT_SOURCE_DIR}/gui/src/")
file (GLOB SOURCE
        "${source_dir}/*.cpp"
        )
add_executable(gui ${SOURCE})

get_target_property(RS_DIR rs LOCATION)

target_link_libraries(gui ${SDL2_LIBRARIES})
target_link_libraries(gui ${RS_DIR}/librs.dylib)

add_dependencies(gui rs)
