#include <fstream>
#include <iostream>
#include <regex>

#include "osu/osz/beatmapparser.hpp"
#include "osu/util/errors.hpp"

namespace osu {

static std::regex rgxLine("(\r?\n)+");
static std::regex rgxSection("\\[([A-Za-z]+)\\]");
static std::regex rgxKeyValuePair("");

BeatmapSet *readString(std::string string) {
    std::smatch match;
    BeatmapSet *set = new BeatmapSet();
    std::sregex_token_iterator it(string.begin(), string.end(), rgxLine, -1);
    std::string currentSection("Version");

    for (std::sregex_token_iterator end; it != end; ++it) {
        auto line = it->str();
        // std::cout << line << std::endl;
        if (std::regex_search(line, match, rgxSection)) {
            currentSection = match[1];
            continue;
        }

        // not a section header
        std::cout << currentSection << std::endl;
        if (!currentSection.compare("Version")) {
        } else if (!currentSection.compare("Events")) {
        } else if (!currentSection.compare("TimingPoints")) {
        } else if (!currentSection.compare("HitObjects")) {
        } else {
            // everything else should be in key-value pair style
        }
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
