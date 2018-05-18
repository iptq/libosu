#pragma once
#ifndef INCLUDE_UTIL_COLOR_HPP_
#define INCLUDE_UTIL_COLOR_HPP_

#include <string>

namespace osu {

class RgbColor {
  public:
    // constructors
    RgbColor(unsigned char red, unsigned char green, unsigned char blue) : red(red), green(green), blue(blue) {}
    RgbColor() : RgbColor(0, 0, 0) {}

    // other useful functions
    std::string hexstring();

  private:
    unsigned char red, green, blue;
};

} // namespace osu

#endif
