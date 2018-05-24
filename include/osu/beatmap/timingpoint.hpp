#pragma once
#ifndef INCLUDE_BEATMAP_TIMINGPOINT_HPP_
#define INCLUDE_BEATMAP_TIMINGPOINT_HPP_

#include "osu/util/common.hpp"
#include "osu/util/enums.hpp"

namespace osu {

class TimingPoint {
  public:
    SampleSet sampleSet;
    uint volume;
    bool kiai;
};

// forward declaration
class InheritedTimingPoint;

class UninheritedTimingPoint : public TimingPoint {
  public:
    InheritedTimingPoint *newChild(uint measure, uint offset, uint division);
    uint offset;
};

class InheritedTimingPoint : public TimingPoint {
  public:
    UninheritedTimingPoint *parent;
    uint measure;
    uint offset;
    uint division;
};

} // namespace osu

#endif
