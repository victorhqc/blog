use crate::{
    authorization::jwt::AuthorizedToken,
    graphql::context::{get_enforcer_from_context, get_token_from_context},
};
use async_graphql::{Context, Error, Guard, Result};
use async_mutex::MutexGuard;
use async_trait::async_trait;
use casbin::{CoreApi, Enforcer};
use snafu::prelude::*;
use std::string::ToString;
use strum_macros::Display;

#[derive(Eq, PartialEq, Display)]
pub enum Resource {
    // Post,
    // Tag,
    User,
    File,
}

#[derive(Eq, PartialEq, Display)]
pub enum Action {
    Read,
    Write,
}

pub struct RoleGuard {
    resource: Option<Resource>,
    action: Option<Action>,
}

impl RoleGuard {
    pub fn new(resource: Resource, action: Action) -> Self {
        RoleGuard {
            resource: Some(resource),
            action: Some(action),
        }
    }

    // pub fn default() -> Self {
    //     RoleGuard {
    //         resource: None,
    //         action: None,
    //     }
    // }
}

impl RoleGuard {
    pub fn authorize_if_needed(
        &self,
        token: &AuthorizedToken,
        e: MutexGuard<Enforcer>,
    ) -> Result<()> {
        let resource = match &self.resource {
            Some(r) => r.to_string().to_lowercase(),
            None => return Ok(()),
        };

        let action = match &self.action {
            Some(a) => a.to_string().to_lowercase(),
            None => return Ok(()),
        };

        let role = token.role.to_lowercase();
        let is_valid = e.enforce((&role, &resource, &action)).unwrap();

        debug!(
            "Is request valid for {:?}? {}",
            (role, resource, action),
            is_valid
        );

        if is_valid {
            Ok(())
        } else {
            Err(Error::from(ClientGuardError::Unauthorized))
        }
    }
}

#[async_trait]
impl Guard for RoleGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let token = get_token_from_context(ctx).await?;
        let enforcer = get_enforcer_from_context(ctx).await?;

        match token {
            Some(token) => {
                debug!("Authorizing Role for Token");
                self.authorize_if_needed(token, enforcer)
            }
            None => {
                debug!("Token missing, unauthenticated");
                Err(Error::from(ClientGuardError::Unauthenticated))
            }
        }
    }
}

#[derive(Debug, Snafu)]
pub enum ClientGuardError {
    #[snafu(display("Unauthenticated, access forbidden."))]
    Unauthenticated,

    #[snafu(display("Unauthorized access."))]
    Unauthorized,
}
