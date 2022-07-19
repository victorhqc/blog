use crate::{
    authorization::graphql::{Action, Resource, RoleGuard},
    graphql::context::get_conn_from_context,
    user::repository::UserRepository,
};
use async_graphql::*;
use entity::{
    enums::Role as UserRole,
    users::{self},
};
use sea_orm::entity::prelude::Uuid;
use std::{convert::From, str::FromStr};

#[derive(SimpleObject, Clone)]
pub struct User {
    pub uuid: ID,
    pub email: String,
    pub role: Role,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard = "RoleGuard::new(Resource::User, Action::Read)")]
    pub async fn user(&self, ctx: &Context<'_>, uuid: String) -> Result<Option<User>> {
        let conn = get_conn_from_context(ctx).await?;

        let uuid = Uuid::from_str(&uuid)?;
        let user = UserRepository::find_by_id(conn, uuid).await?;
        let user = match user {
            Some(u) => u,
            None => return Ok(None),
        };

        Ok(Some(user.into()))
    }
}

#[derive(Enum, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Role {
    Admin,
    Writer,
    Editor,
}

impl From<Role> for UserRole {
    fn from(item: Role) -> Self {
        match item {
            Role::Admin => UserRole::Admin,
            Role::Writer => UserRole::Writer,
            Role::Editor => UserRole::Editor,
        }
    }
}

impl From<UserRole> for Role {
    fn from(item: UserRole) -> Self {
        match item {
            UserRole::Admin => Role::Admin,
            UserRole::Writer => Role::Writer,
            UserRole::Editor => Role::Editor,
        }
    }
}

impl From<users::Model> for User {
    fn from(user: users::Model) -> Self {
        let uuid = Uuid::from_bytes(user.uuid());

        User {
            uuid: uuid.into(),
            email: user.email,
            role: UserRole::from_str(&user.role).unwrap().into(),
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        }
    }
}
