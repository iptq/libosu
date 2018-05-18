#pragma once
#ifndef INCLUDE_BEATMAP_BEATMAP_HPP_
#define INCLUDE_BEATMAP_BEATMAP_HPP_

#include <set>

#include "beatmap/timingpoint.hpp"

namespace osu {

class Beatmap {
  private:
    std::set<TimingPoint> timingPoints_;
};

} // namespace osu

#endif
