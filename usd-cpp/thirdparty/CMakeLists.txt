cmake_minimum_required(VERSION 3.5...3.16)

project(thirdparty)

#### Building TBB
set( TBB_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/oneTBB )

include(${TBB_ROOT}/cmake/TBBBuild.cmake)
tbb_build(TBB_ROOT ${TBB_ROOT} CONFIG_DIR TBB_DIR MAKE_ARGS tbb_build_prefix="")

find_package( TBB )

#### Building boost
add_subdirectory(boost)

#### Building USD

#[===[
cmake::Config::new(usd_root)
        .define("TBB_ROOT_DIR", outdir.display().to_string())
        .define("BOOST_ROOT", outdir.display().to_string())
        .define("PXR_ENABLE_PTEX_SUPPORT", "FALSE")
        .define("PXR_BUILD_IMAGING", "FALSE")
        .define("PXR_ENABLE_PYTHON_SUPPORT", "FALSE")
        .define("PXR_BUILD_TESTS", "FALSE")
]===]

set( PXR_ENABLE_PTEX_SUPPORT FALSE )
set( PXR_BUILD_IMAGING FALSE )
set( PXR_ENABLE_PYTHON_SUPPORT FALSE )
set( PXR_BUILD_TESTS FALSE )
add_subdirectory(USD)

