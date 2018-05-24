#include <fstream>
#include <iostream>
#include <regex>

#include <boost/algorithm/string/trim_all.hpp>

#include "osu/osz/beatmapparser.hpp"
#include "osu/util/errors.hpp"

namespace osu {

static std::regex rgxLine("(\r?\n)+");
static std::regex rgxSection("\\[([A-Za-z]+)\\]");
static std::regex rgxKeyValuePair("(.+)\\:(.+)");

BeatmapSet *readString(std::string string) {
    BeatmapSet *set = new BeatmapSet();
    Beatmap *map = new Beatmap();

    std::smatch match;
    std::sregex_token_iterator it(string.begin(), string.end(), rgxLine, -1);
    std::string currentSection("Version");

    for (std::sregex_token_iterator end; it != end; ++it) {
        auto line = it->str();
        boost::algorithm::trim(line);
        if (line.length() == 0)
            continue;

        // std::cout << line << std::endl;
        if (std::regex_search(line, match, rgxSection)) {
            currentSection = match[1].str();
            continue;
        }

        // not a section header
        // std::cout << currentSection << std::endl;
        if (!currentSection.compare("Version")) {
        } else if (!currentSection.compare("Events")) {
        } else if (!currentSection.compare("TimingPoints")) {
        } else if (!currentSection.compare("Colours")) {
        } else if (!currentSection.compare("HitObjects")) {
        } else {
            // everything else should be in key-value pair style
            if (std::regex_search(line, match, rgxKeyValuePair)) {
                auto key = match[1].str();
                auto value = match[2].str();
                boost::algorithm::trim(key);
                boost::algorithm::trim(value);
                if (!key.compare("AudioFilename")) {
                    map->audioFilename = value;
                } else if (!key.compare("AudioLeadIn")) {
                    map->audioLeadIn = std::stoul(value);
                } else {
                    std::cout << "key: " << key << ", value: " << value << std::endl;
                }
            }
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
