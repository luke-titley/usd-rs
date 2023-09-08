cmake_minimum_required(VERSION 3.5...3.16)

project(thirdparty)

#### Building TBB
set( TBB_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/oneTBB )

include(${TBB_ROOT}/cmake/TBBBuild.cmake)
tbb_build(TBB_ROOT ${TBB_ROOT} CONFIG_DIR TBB_DIR MAKE_ARGS tbb_build_prefix="")

find_package( TBB )

#### Building boost
add_subdirectory(boost)

