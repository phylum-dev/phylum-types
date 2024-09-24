//! This module contains types involved in handling authentication.

use core::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Typed wrapper for AuthorizationCode as used in OAuth login flow with PKCE
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Serialize, Deserialize, JsonSchema,
)]
pub struct AuthorizationCode(String);

impl AuthorizationCode {
    pub fn new(string: impl AsRef<str>) -> Self {
        Self(string.as_ref().to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&AuthorizationCode> for String {
    fn from(val: &AuthorizationCode) -> Self {
        val.0.to_owned()
    }
}

impl fmt::Display for AuthorizationCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for AuthorizationCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Typed wrapper for RefreshToken
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Serialize, Deserialize, JsonSchema,
)]
pub struct RefreshToken(String);

impl RefreshToken {
    pub fn new(string: impl AsRef<str>) -> Self {
        Self(string.as_ref().to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&RefreshToken> for String {
    fn from(val: &RefreshToken) -> Self {
        val.0.to_owned()
    }
}

impl fmt::Display for RefreshToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for RefreshToken {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Typed wrapper for AccessToken
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Serialize, Deserialize, JsonSchema,
)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(string: impl AsRef<str>) -> Self {
        Self(string.as_ref().to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&AccessToken> for String {
    fn from(val: &AccessToken) -> Self {
        val.0.to_owned()
    }
}

impl<'a> From<&'a AccessToken> for &'a str {
    fn from(val: &'a AccessToken) -> Self {
        &val.0
    }
}

impl fmt::Display for AccessToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for AccessToken {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Typed wrapper for IdToken
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Serialize, Deserialize, JsonSchema,
)]
pub struct IdToken(String);

impl IdToken {
    pub fn new(string: impl AsRef<str>) -> Self {
        Self(string.as_ref().to_owned())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&IdToken> for String {
    fn from(val: &IdToken) -> Self {
        val.0.to_owned()
    }
}

impl fmt::Display for IdToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for IdToken {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Represents a response from a OAuth server containing
/// refresh and access tokens obtained from the final stage
/// of the OAuth login flow with PKCE
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Serialize, Deserialize, JsonSchema,
)]
pub struct TokenResponse {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    pub id_token: IdToken,
    #[serde(rename = "expires_in")]
    pub expires_in_seconds: u32,
}

/// Represents a refresh token response from a OAuth server after
/// a request was made to obtain a new Access Token using the current
/// Refresh Token
#[derive(
    PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Serialize, Deserialize, JsonSchema,
)]
pub struct AccessTokenResponse {
    pub access_token: AccessToken,
    #[serde(rename = "expires_in")]
    pub expires_in_seconds: u32,
}
