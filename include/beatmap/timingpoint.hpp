#pragma once
#ifndef INCLUDE_BEATMAP_TIMINGPOINT_HPP_
#define INCLUDE_BEATMAP_TIMINGPOINT_HPP_

#include "common.hpp"

namespace osu {

class UninheritedTimingPoint {
  public:
    InheritedTimingPoint newChild(uint measure, uint offset, uint division);

    uint offset;
};

class InheritedTimingPoint {
  private:
    UninheritedTimingPoint *parent;
};

struct InheritedTimingPointComparator {
    bool operator()(const InheritedTimingPoint &lhs, const InheritedTimingPoint &rhs) { return lhs < rhs; }
};

} // namespace osu

#endif
