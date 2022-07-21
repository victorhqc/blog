use super::{Post, Status};
use crate::{
    authorization::graphql::{Action, Resource, RoleGuard},
    graphql::context::{get_conn_from_context, get_token_from_context},
    posts::{
        ChangePostStatusInput as ChangePostStatusRepoInput, NewPostInput as NewPostRepoInput,
        PostsRepository, UpdatePostInput as UpdatePostRepoInput,
    },
};
use async_graphql::{Context, InputObject, Object, Result as GraphqlResult, ID};
use markdown_to_html::markdown;
use std::str::FromStr;
use uuid::Uuid;

#[derive(InputObject)]
pub struct NewPostInput {
    pub title: String,
    pub raw: String,
}

#[derive(InputObject)]
pub struct UpdatePostInput {
    pub uuid: ID,
    pub title: String,
    pub raw: String,
    pub html: String,
}

#[derive(InputObject)]
pub struct ChangePostStatusInput {
    pub uuid: ID,
    pub status: Status,
}

#[derive(Default)]
pub struct PostsMutation;

#[Object]
impl PostsMutation {
    #[graphql(guard = "RoleGuard::new(Resource::Post, Action::Write)")]
    pub async fn new_post(&self, ctx: &Context<'_>, input: NewPostInput) -> GraphqlResult<Post> {
        let conn = get_conn_from_context(ctx).await?;
        let token = get_token_from_context(ctx).await?.expect("Missing Token");
        let uuid = token.uuid;

        let input = NewPostRepoInput {
            created_by: uuid,
            html: markdown(&input.raw),
            raw: input.raw,
            title: input.title,
        };

        let post = PostsRepository::create(conn, input).await?;
        let post: Post = post.try_into()?;

        Ok(post)
    }

    #[graphql(guard = "RoleGuard::new(Resource::Post, Action::Write)")]
    pub async fn update_post(
        &self,
        ctx: &Context<'_>,
        input: UpdatePostInput,
    ) -> GraphqlResult<Post> {
        let conn = get_conn_from_context(ctx).await?;
        let uuid = Uuid::from_str(&input.uuid)?;

        let input = UpdatePostRepoInput {
            uuid,
            html: input.html,
            raw: input.raw,
            title: input.title,
        };

        let post = PostsRepository::update_post(conn, input).await?;
        let post: Post = post.try_into()?;

        Ok(post)
    }

    #[graphql(guard = "RoleGuard::new(Resource::Post, Action::Write)")]
    pub async fn change_post_status(
        &self,
        ctx: &Context<'_>,
        input: ChangePostStatusInput,
    ) -> GraphqlResult<Post> {
        let conn = get_conn_from_context(ctx).await?;
        let uuid = Uuid::from_str(&input.uuid)?;

        let input = ChangePostStatusRepoInput {
            uuid,
            status: input.status.into(),
        };

        let post = PostsRepository::change_post_status(conn, input).await?;
        let post: Post = post.try_into()?;

        Ok(post)
    }
}
