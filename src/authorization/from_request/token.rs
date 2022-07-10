use super::utils::{try_token_from_request, TryFromReqError};
use crate::authorization::jwt::{JWTError, MaybeToken, Token};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use snafu::prelude::*;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r MaybeToken {
    type Error = TryFromReqError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match try_token_from_request(request) {
            Ok(t) => t,
            Err(err) => {
                let status: Status = match err {
                    TryFromReqError::FailedParsing { source: _ } => Status::InternalServerError,
                    TryFromReqError::MalformedToken => Status::BadRequest,
                };

                return Outcome::Failure((status, err));
            }
        };

        Outcome::Success(request.local_cache(|| MaybeToken(token)))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Token {
    type Error = TokenFromRequestError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match try_token_from_request(request) {
            Ok(t) => t,
            Err(err) => {
                let status: Status = match err {
                    TryFromReqError::FailedParsing { source: _ } => Status::InternalServerError,
                    TryFromReqError::MalformedToken => Status::BadRequest,
                };

                return Outcome::Failure((
                    status,
                    TokenFromRequestError::FailedToGet { source: err },
                ));
            }
        };

        match token {
            Some(token) => Outcome::Success(request.local_cache(|| token)),
            None => Outcome::Failure((Status::Unauthorized, TokenFromRequestError::MissingToken)),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum TokenFromRequestError {
    #[snafu(display("Missing Token"))]
    MissingToken,

    #[snafu(display("Failed to authorize token: {}", source))]
    FailedToAuthorize { source: JWTError },

    #[snafu(display("Failed to get token from request: {}", source))]
    FailedToGet { source: TryFromReqError },
}
