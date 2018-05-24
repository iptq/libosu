#include <fstream>
#include <iostream>
#include <regex>

#include <boost/algorithm/string/trim_all.hpp>

#include "osu/osz/beatmapparser.hpp"
#include "osu/util/errors.hpp"

namespace osu {

static std::regex rgxLine("(\r?\n)+");
static std::regex rgxSection("\\[([A-Za-z]+)\\]");
static std::regex rgxKeyValuePair("(.+)\\:(.*)");

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
            // TODO: handle events
        } else if (!currentSection.compare("TimingPoints")) {
            // TODO: handle timing points
        } else if (!currentSection.compare("Colours")) {
            // TODO: handle colors
        } else if (!currentSection.compare("HitObjects")) {
            // TODO: handle hit objects
        } else {
            // everything else should be in key-value pair style
            if (std::regex_search(line, match, rgxKeyValuePair)) {
                auto key = match[1].str();
                auto value = match[2].str();
                boost::algorithm::trim(key);
                boost::algorithm::trim(value);
                // [General]
                if (!key.compare("AudioFilename")) {
                    map->audioFilename = value;
                } else if (!key.compare("AudioLeadIn")) {
                    map->audioLeadIn = std::stoul(value);
                } else if (!key.compare("PreviewTime")) {
                    set->previewTime = std::stoul(value);
                } else if (!key.compare("Countdown")) {
                    map->countdown = std::stol(value);
                } else if (!key.compare("SampleSet")) {
                    map->sampleSet = SampleSetMapper[value];
                } else if (!key.compare("StackLeniency")) {
                    map->stackLeniency = std::strtod(value.c_str(), NULL);
                } else if (!key.compare("Mode")) {
                    map->mode = (Mode)std::stoul(value);
                } else if (!key.compare("LetterboxInBreaks")) {
                    map->letterboxInBreaks = std::stol(value);
                } else if (!key.compare("WidescreenStoryboard")) {
                    map->widescreenStoryboard = std::stol(value);
                }
                // [Editor]
                else if (!key.compare("Bookmarks")) {
                    // TODO: handle bookmarks
                } else if (!key.compare("DistanceSpacing")) {
                    // TODO: handle distance spacing
                } else if (!key.compare("BeatDivisor")) {
                    // TODO: handle beat divisor
                } else if (!key.compare("GridSize")) {
                    // TODO: handle grid size
                } else if (!key.compare("TimelineZoom")) {
                    // TODO: handle timeline zoom
                }
                // [Metadata]
                else if (!key.compare("Title")) {
                    set->title = value;
                } else if (!key.compare("TitleUnicode")) {
                    set->titleUnicode = value;
                } else if (!key.compare("Artist")) {
                    set->artist = value;
                } else if (!key.compare("ArtistUnicode")) {
                    set->artistUnicode = value;
                } else if (!key.compare("Creator")) {
                    set->mapper = value;
                } else if (!key.compare("Version")) {
                    map->difficultyName = value;
                } else if (!key.compare("Source")) {
                    set->source = value;
                } else if (!key.compare("Tags")) {
                    // TODO: handle tags
                } else if (!key.compare("BeatmapID")) {
                    // TODO: handle beatmap id
                } else if (!key.compare("BeatmapSetID")) {
                    // TODO: handle beatmap set id
                }
                // [Difficulty]
                else if (!key.compare("HPDrainRate")) {
                    map->hpDrainRate = std::strtod(value.c_str(), NULL);
                } else if (!key.compare("CircleSize")) {
                    map->circleSize = std::strtod(value.c_str(), NULL);
                } else if (!key.compare("OverallDifficulty")) {
                    map->overallDifficulty = std::strtod(value.c_str(), NULL);
                } else if (!key.compare("ApproachRate")) {
                    map->approachRate = std::strtod(value.c_str(), NULL);
                } else if (!key.compare("SliderMultiplier")) {
                    map->sliderMultiplier = std::strtod(value.c_str(), NULL);
                } else if (!key.compare("SliderTickRate")) {
                    // TODO: this should be an int representing log2 value since slider tick rate can only be a power of 2
                    map->sliderTickRate = std::strtod(value.c_str(), NULL);
                }
                //
                else {
                    // not really going to care about these for now
                    // std::cout << "key: " << key << ", value: " << value << std::endl;
                }
            }
        }
    }
    set->maps[map->difficultyName] = map;

    return set;
}

BeatmapSet *readFile(std::filesystem::path path) {
    std::ifstream f(path);
    std::stringstream buf;
    buf << f.rdbuf();
    return readString(buf.str());
}

} // namespace osu
