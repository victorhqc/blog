use super::Post;
use crate::{
    authorization::graphql::{Action, Resource, RoleGuard},
    graphql::context::get_conn_from_context,
    posts::PostsRepository,
};
use async_graphql::{Context, Error as GraphqlError, Object, Result as GraphqlResult};
use snafu::prelude::*;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Default)]
pub struct PostsQuery;

#[Object]
impl PostsQuery {
    #[graphql(guard = "RoleGuard::new(Resource::Post, Action::Read)")]
    pub async fn post(&self, ctx: &Context<'_>, uuid: String) -> GraphqlResult<Option<Post>> {
        let conn = get_conn_from_context(ctx).await?;
        let uuid = Uuid::from_str(&uuid)?;

        let post = PostsRepository::find_by_id(conn, uuid).await?;

        match post {
            Some(p) => Ok(Some(p.try_into()?)),
            None => Ok(None),
        }
    }

    #[graphql(guard = "RoleGuard::new(Resource::Post, Action::Read)")]
    pub async fn all_posts(&self, ctx: &Context<'_>) -> GraphqlResult<Vec<Post>> {
        let conn = get_conn_from_context(ctx).await?;

        let posts: Result<Vec<Post>, _> = PostsRepository::find_all(conn)
            .await?
            .into_iter()
            .map(|p| p.try_into())
            .collect();

        match posts {
            Ok(p) => Ok(p),
            Err(err) => Err(GraphqlError::from(err)),
        }
    }
}

#[derive(Debug, Snafu)]
pub enum Error {
    InvalidPost,
}
