#pragma once
#ifndef INCLUDE_UTIL_VECTOR_HPP_
#define INCLUDE_UTIL_VECTOR_HPP_

#include "osu/util/common.hpp"

namespace osu {

template <typename T = uint> class Vector {
  public:
    // constructors
    Vector(T x, T y) : x(x), y(y) {}
    Vector() : Vector(0, 0) {}

    // operators
    friend bool operator==(Vector<T> lhs, const Vector<T> &rhs) { return lhs.x == rhs.x and lhs.y == rhs.y; }
    friend Vector<T> operator+(Vector<T> lhs, const Vector<T> &rhs) { return new Vector(lhs.x + rhs.x, lhs.y + rhs.y); }
    friend Vector<T> operator-(Vector<T> lhs, const Vector<T> &rhs) { return new Vector(lhs.x - rhs.x, lhs.y - rhs.y); }
    friend Vector<T> operator*(Vector<T> lhs, const Vector<T> &rhs) { return new Vector(lhs.x * rhs.x, lhs.y * rhs.y); }
    friend Vector<T> operator*(Vector<T> lhs, T &rhs) { return new Vector(lhs.x * rhs, lhs.y * rhs); }
    friend Vector<T> operator/(Vector<T> lhs, const Vector<T> &rhs) { return new Vector(lhs.x / rhs.x, lhs.y / rhs.y); }

  private:
    T x;
    T y;
};

} // namespace osu

#endif
