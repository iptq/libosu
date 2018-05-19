#include <fstream>
#include <regex>

#include "oldparser/beatmapparser.hpp"

namespace osu {

BeatmapSet *readString(std::string string) {
    BeatmapSet *set = new BeatmapSet();
    return set;
}

BeatmapSet *readFile(std::filesystem::path path) {
    std::ifstream f(path);
    std::stringstream buf;
    buf << f.rdbuf();
    return readString(buf.str());
}

} // namespace osu
