#pragma once
#ifndef INCLUDE_BEATMAP_HITOBJECT_HPP_
#define INCLUDE_BEATMAP_HITOBJECT_HPP_

#include "math/vector.hpp"

namespace osu {

class HitObject {
  private:
    Vector<> startPosition;
};

class HitCircle : public HitObject {};

class Slider : public HitObject {};

class Spinner : public HitObject {};

} // namespace osu

#endif
