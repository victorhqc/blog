use crate::authorization::jwt::Token;
use regex::Regex;
use rocket::{http::hyper::header::AUTHORIZATION, request::Request};
use snafu::prelude::*;

pub fn try_token_from_request(request: &Request<'_>) -> Result<Option<Token>, TryFromReqError> {
    let raw_token = request.headers().get_one(AUTHORIZATION.as_str());
    debug!("Raw token received: {:?}", raw_token);
    let valid_token = match Regex::new(r"Bearer (?P<token>[a-zA-Z0-9-_.]+)") {
        Ok(vt) => vt,
        Err(e) => return Err(TryFromReqError::FailedParsing { source: e }),
    };

    let token = match raw_token {
        Some(t) => {
            let result = valid_token.is_match(t);

            if result {
                let caps = match valid_token.captures(t) {
                    Some(c) => c,
                    None => return Err(TryFromReqError::MalformedToken),
                };

                let token = &caps["token"];

                Some(Token(String::from(token)))
            } else {
                None
            }
        }
        None => None,
    };

    Ok(token)
}

#[derive(Debug, Snafu)]
pub enum TryFromReqError {
    #[snafu(display("Failed to parse token, bad regex"))]
    FailedParsing { source: regex::Error },

    #[snafu(display("Malformed Token"))]
    MalformedToken,
}
