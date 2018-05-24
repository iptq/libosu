#include "osu/beatmap/timingpoint.hpp"

namespace osu {

InheritedTimingPoint *UninheritedTimingPoint::newChild(uint measure, uint offset, uint division) {
    InheritedTimingPoint *itp = new InheritedTimingPoint();
    itp->parent = this;
    itp->measure = measure;
    itp->offset = offset;
    itp->division = division;
    return itp;
}

} // namespace osu
