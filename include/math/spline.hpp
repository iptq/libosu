#pragma once
#ifndef INCLUDE_MATH_SPLINE_HPP_
#define INCLUDE_MATH_SPLINE_HPP_

#include <vector>

#include "math/vector.hpp"

namespace osu {

class Spline {
  public:
    std::vector<Vector<>> points() const { return this->points_; }

  private:
    std::vector<Vector<>> points_;
};

} // namespace osu

#endif
