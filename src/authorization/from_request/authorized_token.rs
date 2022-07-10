use super::utils::{try_token_from_request, TryFromReqError};
use crate::authorization::jwt::{verify_token, AuthorizedToken, JWTError, MaybeAuthorizedToken};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use snafu::prelude::*;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r MaybeAuthorizedToken {
    type Error = AuthorizedTokenFromRequestError;

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
                    AuthorizedTokenFromRequestError::FailedToGet { source: err },
                ));
            }
        };

        let authorized: Option<AuthorizedToken> = match token {
            Some(token) => {
                let authorized = match verify_token(&token) {
                    Ok(a) => a,
                    Err(e) => {
                        return Outcome::Failure((
                            Status::Unauthorized,
                            AuthorizedTokenFromRequestError::FailedToAuthorize { source: e },
                        ))
                    }
                };

                Some(authorized)
            }
            None => None,
        };

        Outcome::Success(request.local_cache(|| MaybeAuthorizedToken(authorized)))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r AuthorizedToken {
    type Error = AuthorizedTokenFromRequestError;

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
                    AuthorizedTokenFromRequestError::FailedToGet { source: err },
                ));
            }
        };

        match token {
            Some(token) => {
                let authorized = match verify_token(&token) {
                    Ok(a) => a,
                    Err(e) => {
                        return Outcome::Failure((
                            Status::Unauthorized,
                            AuthorizedTokenFromRequestError::FailedToAuthorize { source: e },
                        ))
                    }
                };

                Outcome::Success(request.local_cache(|| authorized))
            }
            None => Outcome::Failure((
                Status::Unauthorized,
                AuthorizedTokenFromRequestError::MissingToken,
            )),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum AuthorizedTokenFromRequestError {
    #[snafu(display("Failed to get token from request"))]
    FailedToGet { source: TryFromReqError },

    #[snafu(display("Failed to authorize token"))]
    FailedToAuthorize { source: JWTError },

    #[snafu(display("Token is not in request, unauthorized"))]
    MissingToken,
}
