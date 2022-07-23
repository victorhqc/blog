use super::Tag;
use crate::{
    graphql::loader::DataLoader,
    tags::{Error as TagError, TagsRepository},
};
use async_graphql::{dataloader::Loader, Result};
use snafu::prelude::*;
use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::Arc,
};
use uuid::Uuid;

#[async_trait]
impl Loader<PostTagUuid> for DataLoader {
    type Value = Vec<Tag>;
    type Error = Arc<Error>;

    async fn load(
        &self,
        uuids: &[PostTagUuid],
    ) -> Result<HashMap<PostTagUuid, Self::Value>, Self::Error> {
        let tags = TagsRepository::find_by_post_ids(&self.pool.conn, uuids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<PostTagUuid, Self::Value> = HashMap::new();
        for (post_uuid, tags) in tags.into_iter() {
            let tags = tags.into_iter().map(|t| t.into()).collect();

            grouped.insert(PostTagUuid::new(post_uuid), tags);
        }

        Ok(grouped)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{}", source))]
    QueryError { source: TagError },
}

#[derive(Clone, Eq)]
pub struct PostTagUuid(pub Uuid);

impl PostTagUuid {
    pub fn new(uuid: Uuid) -> Self {
        PostTagUuid(uuid)
    }
}

impl PartialEq for PostTagUuid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for PostTagUuid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
