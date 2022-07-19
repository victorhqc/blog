use crate::{
    authorization::graphql::{Action, Resource, RoleGuard},
    graphql::context::get_conn_from_context,
    user::{
        graphql::{Role, User},
        ChangeRoleInput as ChangeRoleRepositoryInput, UserRepository, UserRepositoryInput,
    },
};
use async_graphql::*;
use entity::enums::Role as UserRole;
use sea_orm::entity::prelude::Uuid;
use snafu::prelude::*;
use std::str::FromStr;

#[derive(InputObject)]
pub struct UserInput {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

#[derive(InputObject)]
pub struct ChangeRoleInput {
    pub uuid: String,
    pub role: Role,
}

#[derive(Default)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn first_user(&self, ctx: &Context<'_>, input: UserInput) -> Result<User> {
        let conn = get_conn_from_context(ctx).await?;

        let amount_users = UserRepository::find_users_amount(conn).await?;

        match amount_users {
            0 => {}
            _ => return Err(Error::from(UserMutationError::AdminAlreadyExists)),
        };

        // The first user must be an admin.
        let input = UserRepositoryInput {
            email: input.email,
            password: input.password,
            password_confirmation: input.password_confirmation,
            role: UserRole::Admin,
        };

        let user = UserRepository::create(conn, input).await?;

        Ok(user.into())
    }

    #[graphql(guard = "RoleGuard::new(Resource::User, Action::Write)")]
    pub async fn new_user(&self, ctx: &Context<'_>, input: UserInput) -> Result<User> {
        let conn = get_conn_from_context(ctx).await?;

        let input = UserRepositoryInput {
            email: input.email,
            password: input.password,
            password_confirmation: input.password_confirmation,
            role: UserRole::Editor,
        };

        let user = UserRepository::create(conn, input).await?;

        Ok(user.into())
    }

    pub async fn change_role(&self, ctx: &Context<'_>, input: ChangeRoleInput) -> Result<User> {
        let conn = get_conn_from_context(ctx).await?;

        let input = ChangeRoleRepositoryInput {
            role: input.role.into(),
            uuid: Uuid::from_str(&input.uuid)?,
        };

        let user = UserRepository::change_role(conn, input).await?;

        Ok(user.into())
    }
}

#[derive(Debug, Snafu)]
pub enum UserMutationError {
    #[snafu(display("Can't create the first user, already registered."))]
    AdminAlreadyExists,
}
