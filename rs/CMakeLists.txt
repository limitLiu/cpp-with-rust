if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

set(RS_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/librs.dylib")

add_custom_target(rs ALL
        COMMENT "Compiling rs module"
        COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
        COMMAND cp ${RS_SO} ${CMAKE_CURRENT_BINARY_DIR}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

set_target_properties(rs PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_test(NAME rs_test
        COMMAND cargo test
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
