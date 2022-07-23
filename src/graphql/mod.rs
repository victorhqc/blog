use crate::{
    authorization::graphql::AuthorizationMutation,
    posts::graphql::{PostsMutation, PostsQuery},
    uploads::graphql::{UploadMutation, UploadQuery},
    user::graphql::{UserMutation, UserQuery},
};
use async_graphql::*;

pub mod context;
mod export_sdl;
pub mod loader;
pub mod routes;

pub use export_sdl::*;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery, PostsQuery, UploadQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(
    UserMutation,
    AuthorizationMutation,
    PostsMutation,
    UploadMutation,
);

pub type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
