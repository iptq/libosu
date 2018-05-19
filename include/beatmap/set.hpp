#pragma once
#ifndef INCLUDE_BEATMAP_SET_HPP_
#define INCLUDE_BEATMAP_SET_HPP_

#include <map>

#include "beatmap/map.hpp"

namespace osu {

class BeatmapSet {
  public:
    bool merge(BeatmapSet *other);

    std::string title;
    std::string titleUnicode;
    std::string artist;
    std::string artistUnicode;
    std::string mapper;
    std::string source;
    std::string tags;
    int beatmapId;
    int beatmapSetId;
    uint previewTime;

    std::map<std::string, Beatmap> maps_;
};

} // namespace osu

#endif
