#pragma once
#ifndef INCLUDE_BEATMAP_MAP_HPP_
#define INCLUDE_BEATMAP_MAP_HPP_

#include <set>

#include "beatmap/timingpoint.hpp"
#include "enums.hpp"

namespace osu {

class Beatmap {
  public:
    std::string audioFilename;
    uint audioLeadIn;
    bool countdown;
    double stackLeniency;
    Mode mode;
    bool letterBoxInBreaks;
    bool widescreenStoryboard;

    std::string difficultyName;
    double hpDrainRate;
    double circleSize;
    double overallDifficulty;
    double approachRate;

    std::set<InheritedTimingPoint> inheritedTimingPoints;
};

} // namespace osu

#endif
