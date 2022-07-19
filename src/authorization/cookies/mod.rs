use crate::authorization::jwt::{verify_token, AuthorizedToken, JWTError, Token};
use rocket::http::CookieJar;
use snafu::prelude::*;

pub fn token_from_cookies(cookies: &CookieJar<'_>) -> Result<AuthorizedToken> {
    let token = cookies
        .get("token")
        .map(|crumb| crumb.value())
        .context(MissingCookieSnafu)?;

    let token = Token(String::from(token));
    let authorized = verify_token(&token).context(InvalidTokenSnafu)?;

    Ok(authorized)
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("MissingCookie"))]
    MissingCookie,

    #[snafu(display("InvalidToken: {}", source))]
    InvalidToken { source: JWTError },
}
