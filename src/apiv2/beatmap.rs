//! Models dealing with beatmaps

use crate::apiv2::data::{RankStatus, Timestamp};
use crate::data::Mode;

#[derive(Debug, Serialize, Deserialize)]
/// Extended beatmap
pub struct Beatmap {
  /// BeatmapCompact
  #[serde(flatten)]
  pub inner: BeatmapCompact,

  /// Overall difficulty
  #[serde(rename = "accuracy")]
  pub overall_difficulty: f64,

  /// Approach rate
  #[serde(rename = "ar")]
  pub approach_rate: f64,

  /// Beatmapset id
  pub beatmapset_id: i32,

  /// Overall representative bpm
  pub bpm: f64,

  /// TODO: what is this value?
  pub convert: bool,

  /// Number of circles in the map
  pub count_circles: usize,

  /// Number of sliders in the map
  pub count_sliders: usize,

  /// Number of spinners in the map
  pub count_spinners: usize,

  /// Circle size
  #[serde(rename = "cs")]
  pub circle_size: f64,

  /// Date/time the map was deleted (if it was)
  pub deleted_at: Option<Timestamp>,

  /// HP Drain
  #[serde(rename = "drain")]
  pub hp_drain: f64,

  /// Drain time (in seconds)
  #[serde(rename = "hit_length")]
  pub drain_length: u32,

  /// TODO: what is this?
  pub is_scoreable: bool,

  /// Last updated
  pub last_updated: Timestamp,

  /// Mode represented as an integer
  pub mode_int: u32,

  /// Amount of passes
  #[serde(rename = "passcount")]
  pub pass_count: u32,

  /// Amount of plays
  #[serde(rename = "playcount")]
  pub play_count: u32,

  /// Ranked status
  pub ranked: u32,

  /// URL to the beatmap
  pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Beatmap
pub struct BeatmapCompact {
  /// Star rating of the beatmap
  pub difficulty_rating: f64,

  /// Beatmap id
  pub id: i32,

  /// Game mode
  pub mode: Mode,

  /// Rank status
  pub status: RankStatus,

  /// Total length (number of seconds)
  pub total_length: u32,

  /// Difficulty name
  #[serde(rename = "version")]
  pub difficulty_name: String,

  /// Beatmapset
  pub beatmapset: Option<Beatmapset>,

  /// Checksum (MD5)
  pub checksum: Option<String>,

  /// Failtimes
  pub failtimes: Option<Failtimes>,

  /// Max combo that can be achieved in the map
  pub max_combo: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Failtimes
///
/// All fields are optional but there's always at least one field returned.
pub struct Failtimes {
  /// Frequency of exits at a given percent through the map
  pub exit: Option<Vec<i32>>,

  /// Frequency of fails at a given percent through the map
  pub fail: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
/// Beatmapset
pub struct Beatmapset {}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_apiv2_serde() {
    const s: &str = r#"{"difficulty_rating":5.86,"id":2498577,"mode":"osu","status":"ranked","total_length":87,"version":"Special","accuracy":9,"ar":8.5,"beatmapset_id":1199834,"bpm":204,"convert":false,"count_circles":179,"count_sliders":103,"count_spinners":0,"cs":4,"deleted_at":null,"drain":5.2,"hit_length":85,"is_scoreable":true,"last_updated":"2020-10-30T15:04:53+00:00","mode_int":0,"passcount":2084,"playcount":30331,"ranked":1,"url":"https:\/\/osu.ppy.sh\/beatmaps\/2498577","checksum":"16a14ee9f54f2c3a7bd06faec9e572ec","beatmapset":{"artist":"Vickeblanka","artist_unicode":"ビッケブランカ","covers":{"cover":"https:\/\/assets.ppy.sh\/beatmaps\/1199834\/covers\/cover.jpg?1604070309","cover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/1199834\/covers\/cover@2x.jpg?1604070309","card":"https:\/\/assets.ppy.sh\/beatmaps\/1199834\/covers\/card.jpg?1604070309","card@2x":"https:\/\/assets.ppy.sh\/beatmaps\/1199834\/covers\/card@2x.jpg?1604070309","list":"https:\/\/assets.ppy.sh\/beatmaps\/1199834\/covers\/list.jpg?1604070309","list@2x":"https:\/\/assets.ppy.sh\/beatmaps\/1199834\/covers\/list@2x.jpg?1604070309","slimcover":"https:\/\/assets.ppy.sh\/beatmaps\/1199834\/covers\/slimcover.jpg?1604070309","slimcover@2x":"https:\/\/assets.ppy.sh\/beatmaps\/1199834\/covers\/slimcover@2x.jpg?1604070309"},"creator":"IOException","favourite_count":181,"hype":null,"id":1199834,"nsfw":false,"play_count":229037,"preview_url":"\/\/b.ppy.sh\/preview\/1199834.mp3","source":"ブラッククローバー","status":"ranked","title":"Black Rover (TV Size)","title_unicode":"Black Rover (TV Size)","user_id":2688103,"video":false,"availability":{"download_disabled":false,"more_information":null},"bpm":204,"can_be_hyped":false,"discussion_enabled":true,"discussion_locked":false,"is_scoreable":true,"last_updated":"2020-10-30T15:04:52+00:00","legacy_thread_url":"https:\/\/osu.ppy.sh\/community\/forums\/topics\/1094074","nominations_summary":{"current":2,"required":2},"ranked":1,"ranked_date":"2020-11-06T07:04:56+00:00","storyboard":false,"submitted_date":"2020-06-24T07:32:02+00:00","tags":"anime rock japanese clover opening op deadcode ceo of osu ceo_of_osu","ratings":[0,54,4,4,3,7,2,10,17,30,170]},"failtimes":{"fail":[0,0,0,0,0,0,0,0,0,2,74,959,307,1447,1480,336,1505,4063,1203,767,209,36,166,29,10,31,3,15,22,20,57,80,39,45,28,46,29,10,132,47,18,74,18,1,5,2,0,8,7,10,21,1,12,0,26,34,49,68,63,261,442,216,85,144,50,307,134,52,67,48,237,330,120,42,14,27,159,469,200,260,171,175,23,41,57,187,166,152,9,22,34,5,9,2,0,0,0,1,1,0],"exit":[0,0,0,0,0,0,0,0,5,209,323,2605,983,453,1185,180,271,1071,681,322,744,113,105,142,14,75,56,24,98,48,55,182,92,62,42,70,79,31,55,96,24,35,134,30,16,14,7,13,16,7,27,17,4,2,2,3,7,10,20,19,38,47,14,3,3,13,41,9,9,5,5,12,22,3,3,3,6,8,13,5,15,6,5,3,5,7,16,64,141,41,34,24,21,15,14,17,15,33,115,1]},"max_combo":575}"#;
    let x: Result<Beatmap, _> = serde_json::from_str(s);
    assert!(x.is_ok());
  }
}
