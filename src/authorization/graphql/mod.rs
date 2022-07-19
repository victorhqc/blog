use async_graphql::*;

mod mutation;
mod role_guard;

pub use mutation::*;
pub use role_guard::*;

#[derive(Clone, SimpleObject)]
pub struct Token {
    pub token: String,
}
