cmake_minimum_required(VERSION 3.0)
project(osu CXX)

include_directories(include)
add_library(
    osu SHARED
    src/beatmap/hitobject.cpp
    src/math/spline.cpp
    src/util/color.cpp
)

install(
    TARGETS osu
    LIBRARY DESTINATION lib
)
