//! osu! API v1
//! -----------
//!
//! Documentation: https://github.com/ppy/osu-api/wiki

use std::fmt;

use futures::stream::TryStreamExt;
use hyper::{
    client::{Client, HttpConnector},
    Body,
};
use hyper_tls::HttpsConnector;

use crate::Mode;

const API_BASE: &str = "https://osu.ppy.sh/api";

/// An error that could occur when using the API.
#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum APIError {
    #[error("hyper error: {0}")]
    Hyper(#[from] hyper::Error),

    #[error("serde_json error: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T, E = APIError> = std::result::Result<T, E>;

/// A struct used for interfacing with the osu! API.
pub struct API {
    client: Client<HttpsConnector<HttpConnector>>,
    api_key: String,
}

impl API {
    /// Creates a new API object with the specified key.
    /// Obtain a key from the [osu! website](https://osu.ppy.sh/p/api).
    pub fn new(api_key: impl AsRef<str>) -> Self {
        let https = HttpsConnector::new();
        API {
            client: Client::builder().build::<_, Body>(https),
            api_key: api_key.as_ref().to_owned(),
        }
    }

    /// Gets all recent
    pub async fn get_user_recent(
        &self,
        user: impl Into<UserLookup>,
        mode: Option<Mode>,
        limit: Option<u32>,
    ) -> Result<Vec<UserScore>> {
        let user = user.into();
        let mode = match mode {
            Some(n) => n as u32,
            None => 0,
        };

        let uri = format!(
            "{}/get_user_recent?k={}&u={}&m={}&limit={}",
            API_BASE,
            self.api_key,
            user,
            mode,
            limit.unwrap_or(10),
        )
        .parse()
        .unwrap();

        let mut resp = self.client.get(uri).await?;
        let body = resp.body_mut();
        // TODO: is there a more elegant way to do this
        let mut result = Vec::new();
        loop {
            let next = match body.try_next().await {
                Ok(Some(v)) => v,
                Ok(None) => break,
                Err(e) => return Err(e.into()),
            };
            result.extend(next);
        }
        serde_json::from_slice(&result).map_err(APIError::from)
    }
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

/// A method of looking up a user in the API.
pub enum UserLookup {
    /// Look up by ID
    Id(u32),

    /// Look up by username
    Name(String),
}

impl fmt::Display for UserLookup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserLookup::Id(n) => write!(f, "{}", n),
            UserLookup::Name(s) => write!(f, "{}", s),
        }
    }
}

impl From<&str> for UserLookup {
    fn from(s: &str) -> Self {
        UserLookup::Name(s.to_owned())
    }
}

impl From<u32> for UserLookup {
    fn from(n: u32) -> Self {
        UserLookup::Id(n)
    }
}

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
