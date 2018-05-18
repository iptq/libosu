#pragma once
#ifndef INCLUDE_UTIL_VECTOR_HPP_
#define INCLUDE_UTIL_VECTOR_HPP_

#include "common.hpp"

namespace osu {

template <typename T = uint> class Vector {
  public:
    // constructors
    Vector(T x, T y) : x(x), y(y) {}
    Vector() : Vector(0, 0) {}

    // operators
    friend Vector<T> operator+(Vector<T> lhs, const Vector<T> &rhs) { return new Vector(lhs.x + rhs.x, lhs.y + rhs.y); }

  private:
    T x;
    T y;
};

} // namespace osu

#endif
