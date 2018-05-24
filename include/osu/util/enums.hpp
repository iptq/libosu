#pragma once
#ifndef INCLUDE_ENUMS_HPP_
#define INCLUDE_ENUMS_HPP_

namespace osu {

enum Mode {
    Standard = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
};

enum SampleSet {
    Auto = 0,
    Normal = 1,
    Soft = 2,
    Drum = 3,
};

enum Additions {
    None = 1,
    Whistle = 2,
    Finish = 4,
    Clap = 8,
};

} // namespace osu

#endif
