cmake_minimum_required(VERSION 3.16)

project(buildTbb CXX)

set(ONE_TBB ${CMAKE_SOURCE_DIR}/oneTBB)

include(${ONE_TBB}/cmake/TBBBuild.cmake)
tbb_build(TBB_ROOT ${ONE_TBB} CONFIG_DIR TBB_DIR MAKE_ARGS "extra_inc=big_iron.inc")
