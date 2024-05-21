//! General data

use chrono::{DateTime, Utc};

/// Timestamp
pub type Timestamp = DateTime<Utc>;

#[derive(Debug, Serialize, Deserialize)]
#[repr(i32)]
/// Possible ranked statuses
pub enum RankStatus {
  /// Graveyard
  #[serde(rename = "graveyard")]
  Graveyard = -2,

  /// WIP
  #[serde(rename = "wip")]
  Wip = -1,

  /// Pending
  #[serde(rename = "pending")]
  Pending = 0,

  /// Ranked
  #[serde(rename = "ranked")]
  Ranked = 1,

  /// Approved
  #[serde(rename = "approved")]
  Approved = 2,

  /// Qualified
  #[serde(rename = "qualified")]
  Qualified = 3,

  /// Loved
  #[serde(rename = "loved")]
  Loved = 4,
}
