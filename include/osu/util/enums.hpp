#pragma once
#ifndef INCLUDE_ENUMS_HPP_
#define INCLUDE_ENUMS_HPP_

#include <map>

namespace osu {

enum Mode {
    ModeStandard = 0,
    ModeTaiko = 1,
    ModeCatch = 2,
    ModeMania = 3,
};

enum SampleSet {
    SampleAuto = 0,
    SampleNormal = 1,
    SampleSoft = 2,
    SampleDrum = 3,
};

static std::map<std::string, SampleSet> SampleSetMapper = {
    std::pair<std::string, SampleSet>("Auto", SampleAuto),
    std::pair<std::string, SampleSet>("Normal", SampleNormal),
    std::pair<std::string, SampleSet>("Soft", SampleSoft),
    std::pair<std::string, SampleSet>("Drum", SampleDrum),
};

enum Additions {
    AdditionNone = 1,
    AdditionWhistle = 2,
    AdditionFinish = 4,
    AdditionClap = 8,
};

} // namespace osu

#endif
