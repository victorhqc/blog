use crate::{
    graphql::loader::DataLoader,
    user::{
        graphql::User,
        repository::{Error as UserError, UserRepository},
    },
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

#[derive(Clone, Eq)]
pub struct UserUuid(pub Uuid);

impl UserUuid {
    pub fn new(uuid: Uuid) -> Self {
        UserUuid(uuid)
    }
}

impl PartialEq for UserUuid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for UserUuid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[async_trait]
impl Loader<UserUuid> for DataLoader {
    type Value = User;
    type Error = Arc<Error>;

    async fn load(
        &self,
        uuids: &[UserUuid],
    ) -> Result<HashMap<UserUuid, Self::Value>, Self::Error> {
        let users = UserRepository::find_by_ids(&self.pool.conn, uuids)
            .await
            .context(QuerySnafu)?;

        let mut grouped: HashMap<UserUuid, Self::Value> = HashMap::new();
        for user in users.into_iter() {
            grouped.insert(UserUuid::new(Uuid::from_bytes(user.uuid())), user.into());
        }

        Ok(grouped)
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Something went wrong with the query: {}", source))]
    QueryError { source: UserError },
}
