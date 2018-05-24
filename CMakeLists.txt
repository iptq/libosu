cmake_minimum_required(VERSION 3.0)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
project(osu CXX)

include_directories(include)

add_library(
    osu SHARED
    src/beatmap/hitobject.cpp
    src/beatmap/set.cpp
    src/beatmap/timingpoint.cpp
    src/math/spline.cpp
    src/osz/beatmapparser.cpp
    src/util/color.cpp
)

install(
    TARGETS osu
    INCLUDES DESTINATION include
    LIBRARY DESTINATION lib
)
