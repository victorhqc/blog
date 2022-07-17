use async_graphql::{Enum, SimpleObject, ID};
use chrono::{DateTime, Utc};
use entity::{enums::Status as DBStatus, posts};
use snafu::prelude::*;
use std::{convert::From, str::FromStr};
use strum::ParseError;
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(SimpleObject, Clone)]
pub struct Post {
    pub uuid: ID,
    pub status: Status,
    pub raw: String,
    pub html: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // #[graphql(skip)]
    // created_by: String,
}

impl TryFrom<posts::Model> for Post {
    type Error = Error;

    fn try_from(post: posts::Model) -> Result<Self, Self::Error> {
        let uuid = Uuid::from_bytes(post.uuid());
        let status = Status::from_str(&post.status).context(InvalidStatusSnafu)?;

        Ok(Post {
            uuid: uuid.into(),
            status,
            html: post.html,
            raw: post.raw,
            created_at: post.created_at,
            updated_at: post.updated_at,
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
    #[snafu(display("Given String is not a valid status: {}", source))]
    InvalidStatus { source: ParseError },
}
