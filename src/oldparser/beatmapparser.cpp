#include <fstream>
#include <regex>

#include "osu/oldparser/beatmapparser.hpp"
#include "osu/util/errors.hpp"

namespace osu {

static std::regex rgxLine("(\r?\n)+");

BeatmapSet *readString(std::string string) {
    BeatmapSet *set = new BeatmapSet();
    std::sregex_token_iterator it(string.begin(), string.end(), rgxLine, -1);

    for (std::sregex_token_iterator end; it != end; ++it) {
        printf("shiet '%s'\n", it->str().c_str());
    }

    return set;
}

BeatmapSet *readFile(std::filesystem::path path) {
    std::ifstream f(path);
    std::stringstream buf;
    buf << f.rdbuf();
    return readString(buf.str());
}

} // namespace osu
