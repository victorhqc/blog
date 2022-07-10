use super::Token as GraphqlToken;
use crate::authorization::jwt::{sign_token, Token};
use crate::graphql::context::get_pool_from_context;
use crate::user::UserRepository;
use async_graphql::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(InputObject, Clone, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Default)]
pub struct AuthorizationMutation;

#[Object]
impl AuthorizationMutation {
    pub async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<GraphqlToken> {
        let pool = get_pool_from_context(ctx).await.unwrap();

        let user =
            UserRepository::find_by_credentials(&pool.conn, input.email, input.password).await?;

        let token = sign_token(user)?;

        Ok(token.into())
    }
}

impl From<Token> for GraphqlToken {
    fn from(token: Token) -> Self {
        GraphqlToken {
            token: token.into(),
        }
    }
}
