//! pp calculation

use rosu_pp::Beatmap as RosuBeatmap;

use super::Beatmap;

impl Beatmap {
  /// Convert to rosu_pp::Beatmap
  pub fn convert_to_rosu_beatmap(&self) -> rosu_pp::ParseResult<RosuBeatmap> {
    let contents = format!("{}", self);
    RosuBeatmap::parse(contents.as_bytes())
  }
}
