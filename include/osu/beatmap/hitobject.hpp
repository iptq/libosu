#pragma once
#ifndef INCLUDE_BEATMAP_HITOBJECT_HPP_
#define INCLUDE_BEATMAP_HITOBJECT_HPP_

#include <vector>

#include "osu/beatmap/timingpoint.hpp"
#include "osu/math/vector.hpp"

namespace osu {

class HitObject {
  public:
    Vector<> startPosition;

    UninheritedTimingPoint *parent;
    uint measure;
    uint offset;
    uint division;
};

class HitCircle : public HitObject {
  public:
    std::vector<Vector<>> controlPoints;
};

class Slider : public HitObject {};

class Spinner : public HitObject {};

} // namespace osu

#endif
