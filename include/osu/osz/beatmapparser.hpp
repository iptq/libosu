#pragma once
#ifndef INCLUDE_OSZ_BEATMAP_HPP_
#define INCLUDE_OSZ_BEATMAP_HPP_

#include <filesystem>
#include <string>

#include "osu/beatmap/set.hpp"

namespace osu {

BeatmapSet *readString(std::string string);
BeatmapSet *readFile(std::filesystem::path path);

} // namespace osu

#endif
