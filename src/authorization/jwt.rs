use chrono::{DateTime, Utc};
use entity::users::Model as User;
use hmac::{Hmac, Mac};
use jwt::{Error, SignWithKey, VerifyWithKey};
use sea_orm::prelude::Uuid;
use sha2::Sha512;
use snafu::prelude::*;
use std::{collections::BTreeMap, env, str::FromStr};

#[derive(Debug, Clone)]
pub struct Token(pub String);

#[derive(Debug, Clone)]
pub struct MaybeToken(pub Option<Token>);

#[derive(Debug, Clone)]
pub struct AuthorizedToken {
    pub uuid: Uuid,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MaybeAuthorizedToken(pub Option<AuthorizedToken>);

pub fn sign_token(user: User) -> Result<Token> {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY is not defined");
    let key: Hmac<Sha512> = Hmac::new_from_slice(secret.as_bytes()).context(KeyFailedSnafu)?;
    let now: DateTime<Utc> = Utc::now();

    let mut claims = BTreeMap::<String, String>::new();
    let uuid = Uuid::from_bytes(user.uuid());
    claims.insert("uuid".to_string(), uuid.to_string());
    claims.insert("role".to_string(), user.role);
    claims.insert("created_at".to_string(), now.to_string());

    let token = claims.sign_with_key(&key).context(SignatureFailedSnafu)?;

    Ok(Token(token))
}

pub fn verify_token(token: &impl GetToken) -> Result<AuthorizedToken> {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY is not defined");
    let duration_min = env::var("TOKEN_DURATION_MIN").expect("TOKEN_DURATION_MIN is not defined");
    let duration_min: i64 = duration_min
        .parse::<i64>()
        .expect("TOKEN_DURATION_MIN is invalid");

    let key: Hmac<Sha512> = Hmac::new_from_slice(secret.as_bytes()).context(KeyFailedSnafu)?;

    let token = token.get_token()?;

    let claims: BTreeMap<String, String> = token
        .verify_with_key(&key)
        .context(TokenVerificationFailedSnafu)?;

    let created_at = DateTime::from_str(&claims["created_at"]).context(InvalidDateSnafu)?;
    let now: DateTime<Utc> = Utc::now();

    let elapsed = now - created_at;
    let remaining = duration_min - elapsed.num_minutes();
    debug!("token emitted {} minutes ago", elapsed.num_minutes());

    if remaining < 0 {
        return Err(JWTError::TokenExpired);
    }
    debug!("token is valid for {} more minutes", remaining);

    let uuid: Uuid = Uuid::from_str(claims["uuid"].as_str()).context(InvalidUuidSnafu)?;

    Ok(AuthorizedToken {
        role: claims["role"].clone(),
        uuid,
        created_at,
    })
}

pub type Result<T, E = JWTError> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum JWTError {
    #[snafu(display("JWT Key failed: {}", source))]
    KeyFailed { source: sha2::digest::InvalidLength },

    #[snafu(display("JWT Signature failed: {}", source))]
    SignatureFailed { source: Error },

    #[snafu(display("JWT Token verification failed: {}", source))]
    TokenVerificationFailed { source: Error },

    #[snafu(display("The JWT Token is invalid, some data is missing"))]
    InvalidToken,

    #[snafu(display("JWT Token has an invalid date: {}", source))]
    InvalidDate { source: chrono::ParseError },

    #[snafu(display("JWT Token is missing"))]
    TokenMissing,

    #[snafu(display("JWT Token has expired"))]
    TokenExpired,

    #[snafu(display("Uuid in JWT Token is invalid: {}", source))]
    InvalidUuid { source: uuid::Error },
}

impl From<Token> for String {
    fn from(t: Token) -> Self {
        t.0
    }
}

pub trait GetToken {
    fn get_token(&self) -> Result<&str, JWTError>;
}

impl GetToken for Token {
    fn get_token(&self) -> Result<&str, JWTError> {
        Ok(&self.0)
    }
}

impl GetToken for MaybeToken {
    fn get_token(&self) -> Result<&str, JWTError> {
        let token = match &self.0 {
            Some(t) => t,
            None => return Err(JWTError::TokenMissing),
        };

        Ok(&token.0)
    }
}
