#pragma once
#ifndef INCLUDE_BEATMAP_SET_HPP_
#define INCLUDE_BEATMAP_SET_HPP_

#include <map>

#include "beatmap/beatmap.hpp"

namespace osu {

class BeatmapSet {
  private:
    std::map<std::string, Beatmap> maps_;
};

} // namespace osu

#endif
