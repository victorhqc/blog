use crate::{
    authorization::graphql::AuthorizationMutation,
    user::graphql::{UserMutation, UserQuery},
};
use async_graphql::*;

pub mod context;
mod export_sdl;
pub mod loader;
pub mod routes;

pub use export_sdl::*;

#[derive(MergedObject, Default)]
pub struct QueryRoot(UserQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(UserMutation, AuthorizationMutation);

pub type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;
