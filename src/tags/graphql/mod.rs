use async_graphql::{SimpleObject, ID};
use entity::tags;
use std::convert::From;
use uuid::Uuid;

mod loader;

pub use loader::*;

#[derive(SimpleObject, Clone, Debug)]
pub struct Tag {
    pub uuid: ID,
    pub name: String,
}

impl From<tags::Model> for Tag {
    fn from(tag: tags::Model) -> Self {
        let uuid = Uuid::from_bytes(tag.uuid());

        Tag {
            uuid: uuid.into(),
            name: tag.name,
        }
    }
}
