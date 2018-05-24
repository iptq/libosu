#pragma once
#ifndef INCLUDE_BEATMAP_MAP_HPP_
#define INCLUDE_BEATMAP_MAP_HPP_

#include <set>

#include "osu/beatmap/timingpoint.hpp"
#include "osu/util/enums.hpp"

namespace osu {

class Beatmap {
  public:
    std::string audioFilename;
    uint audioLeadIn;
    bool countdown;
    SampleSet sampleSet;
    double stackLeniency;
    Mode mode;
    bool letterboxInBreaks;
    bool widescreenStoryboard;

    std::string difficultyName;
    double hpDrainRate;
    double circleSize;
    double overallDifficulty;
    double approachRate;
    double sliderMultiplier;
    double sliderTickRate;

    std::set<InheritedTimingPoint> inheritedTimingPoints;
};

} // namespace osu

#endif
