/// The approved status of a beatmap.
#[allow(missing_docs)]
#[derive(Debug)]
pub enum ApprovedStatus {
  StatusGraveyard,
  StatusWIP,
  StatusPending,
  StatusRanked,
  StatusApproved,
  StatusQualified,
  StatusLoved,
}

/// A score as returned from the osu! API.
#[derive(Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct UserScore {
  pub beatmap_id: String,
  pub score: String,
  pub max_combo: String,
  pub count_50: String,
  pub count_100: String,
  pub count_300: String,
  pub count_miss: String,
  pub count_katu: String,
  pub count_geki: String,
  pub perfect: String,
  pub enabled_mods: String,
  pub user_id: String,
  pub date: String,
  pub rank: String,
}
