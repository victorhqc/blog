use super::Post;
use crate::{
    authorization::graphql::{Action, Resource, RoleGuard},
    graphql::context::{get_conn_from_context, get_token_from_context},
    posts::{NewPostInput as NewPostRepoInput, PostsRepository},
};
use async_graphql::{Context, InputObject, Object, Result as GraphqlResult};

#[derive(InputObject)]
pub struct NewPostInput {
    pub title: String,
    pub raw: String,
    pub html: String,
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
            html: input.html,
            raw: input.raw,
            title: input.title,
        };

        let post = PostsRepository::create(conn, input).await?;
        let post = post.try_into()?;

        Ok(post)
    }
}
