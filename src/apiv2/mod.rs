//! Structs for interacting with APIv2
//!
//! API Documentation: <https://osu.ppy.sh/docs>
//!

pub mod beatmap;
pub mod data;

use reqwest::Client;

const TOKEN_URL: &str = "https://osu.ppy.sh/oauth/token";

/// APIv2
pub struct Apiv2 {
  client_id: u32,
  client_secret: String,
  client: Client,
}

/// Errors that could occur when using the API
#[derive(Debug, Error)]
pub enum ApiError {
  /// Reqwest library error
  #[error("reqwest error: {0}")]
  Reqwest(#[from] reqwest::Error),
}

impl Apiv2 {
  /// Perform a client-credentials token request
  pub async fn client_credentials_grant(
    &self,
  ) -> Result<ClientCredentialsToken, ApiError> {
    #[derive(Serialize)]
    struct TokenRequest<'a> {
      client_id: u32,
      client_secret: &'a str,
      grant_type: &'static str,
      scope: &'static str,
    }

    let req = self
      .client
      .post(TOKEN_URL)
      .json(&TokenRequest {
        client_id: self.client_id,
        client_secret: &self.client_secret,
        grant_type: "client_credentials",
        scope: "public",
      })
      .build()?;

    Ok(self.client.execute(req).await?.json().await?)
  }

  /// Makes a request for an authorization code; this occurs as the second step of the OAuth
  /// verification flow, after the user has already visited the osu website and authorized your
  /// application to use their data.
  pub async fn authorization_code_grant(
    &self,
    code: impl AsRef<str>,
    redirect_uri: impl AsRef<str>,
  ) -> Result<AuthorizationCodeToken, ApiError> {
    let code = code.as_ref();
    let redirect_uri = redirect_uri.as_ref();

    #[derive(Serialize)]
    struct TokenRequest<'a, 'b, 'c> {
      client_id: u32,
      client_secret: &'a str,
      code: &'b str,
      grant_type: &'static str,
      redirect_uri: &'c str,
    }

    let req = self
      .client
      .post(TOKEN_URL)
      .json(&TokenRequest {
        client_id: self.client_id,
        client_secret: &self.client_secret,
        code,
        grant_type: "authorization_code",
        redirect_uri,
      })
      .build()?;

    Ok(self.client.execute(req).await?.json().await?)
  }
}

#[derive(Deserialize)]
/// The token retrieved from a client-credentials grant
pub struct ClientCredentialsToken {
  /// The type of token, this should always be `Bearer`.
  pub token_type: String,
  /// The number of seconds the token will be valid for.
  pub expires_in: u64,
  /// The access token.
  pub access_token: String,
}

#[derive(Deserialize)]
/// The token retrieved from an authorization-code grant
pub struct AuthorizationCodeToken {
  /// The type of token, this should always be `Bearer`.
  pub token_type: String,
  /// The number of seconds the token will be valid for.
  pub expires_in: u64,
  /// The access token.
  pub access_token: String,
  /// The refresh token.
  pub refresh_token: String,
}
