#pragma once
#ifndef INCLUDE_UTIL_COLOR_HPP_
#define INCLUDE_UTIL_COLOR_HPP_

#include <string>

#include "common.hpp"

namespace osu {

class RgbColor {
  public:
    // constructors
    RgbColor(uchar red, uchar green, uchar blue) : red(red), green(green), blue(blue) {}
    RgbColor() : RgbColor(0, 0, 0) {}

    // other useful functions
    std::string hexstring();

    // actual colors
    uchar red, green, blue;
};

} // namespace osu

#endif
