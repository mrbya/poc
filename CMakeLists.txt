cmake_minimum_required(VERSION 3.20.0)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})
project(poc LANGUAGES C CXX)
rust_cargo_application()

set(srcDir ${CMAKE_CURRENT_SOURCE_DIR}/platform/src)
FILE(GLOB commonSources ${srcDir}/common/*.cpp)
FILE(GLOB drvBackend ${srcDir}/backends/qemu/*.cpp)

target_sources(app PRIVATE
  ${commonSources}
  ${drvBackend}
)
target_include_directories(app PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/platform/include)

add_custom_target(
    fix
    COMMAND
        ${CMAKE_SOURCE_DIR}/fix_glibcpp.py
)

add_dependencies(app fix)

