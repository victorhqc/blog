use crate::{
    graphql::loader::DataLoader as AppLoader,
    user::graphql::{User, UserUuid},
};
use async_graphql::{
    dataloader::DataLoader, ComplexObject, Context, Enum, Result, SimpleObject, ID,
};
use chrono::{DateTime, Utc};
use entity::{enums::Status as DBStatus, posts};
use snafu::prelude::*;
use std::{convert::From, str::FromStr};
use strum::ParseError;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

mod mutation;
mod query;

pub use mutation::*;
pub use query::*;

#[derive(SimpleObject, Clone, Debug)]
#[graphql(complex)]
pub struct Post {
    pub uuid: ID,
    pub status: Status,
    pub raw: String,
    pub html: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[graphql(skip)]
    created_by: String,
}

#[ComplexObject]
impl Post {
    async fn author(&self, ctx: &Context<'_>) -> Result<User> {
        let loader = ctx.data_unchecked::<DataLoader<AppLoader>>();
        let uuid = Uuid::from_str(&self.created_by)?;
        let uuid = UserUuid::new(uuid);
        let author = loader.load_one(uuid).await?.context(InvalidAuthorSnafu)?;

        Ok(author)
    }
}

#[derive(Debug, Snafu)]
pub enum GraphqlError {
    #[snafu(display("Post has Invalid author"))]
    InvalidAuthor,
}

impl TryFrom<posts::Model> for Post {
    type Error = Error;

    fn try_from(post: posts::Model) -> Result<Self, Self::Error> {
        let uuid = Uuid::from_bytes(post.uuid());
        let created_by = Uuid::from_bytes(post.uuid());
        let status = Status::from_str(&post.status).context(InvalidStatusSnafu { uuid })?;

        Ok(Post {
            uuid: uuid.into(),
            status,
            html: post.html,
            raw: post.raw,
            created_at: post.created_at,
            updated_at: post.updated_at,
            created_by: created_by.to_string(),
        })
    }
}

#[derive(Enum, Clone, Copy, Debug, Eq, PartialEq, Display, EnumString)]
pub enum Status {
    Published,
    Disabled,
    Draft,
}

impl From<Status> for DBStatus {
    fn from(status: Status) -> Self {
        match status {
            Status::Published => DBStatus::Published,
            Status::Disabled => DBStatus::Disabled,
            Status::Draft => DBStatus::Draft,
        }
    }
}

impl From<DBStatus> for Status {
    fn from(status: DBStatus) -> Self {
        match status {
            DBStatus::Published => Status::Published,
            DBStatus::Disabled => Status::Disabled,
            DBStatus::Draft => Status::Draft,
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Status: {} is not valid for Post: {}", source, uuid))]
    InvalidStatus { source: ParseError, uuid: Uuid },
}
