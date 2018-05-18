#include <sstream>

#include "util/color.hpp"

namespace osu {

std::string RgbColor::hexstring() {
    std::ostringstream stream;
    stream << std::hex << this->red << this->green << this->blue;
    return stream.str();
}

} // namespace osu
