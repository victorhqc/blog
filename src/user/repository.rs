use crate::{
    authorization::password::{encrypt_password, verify_password, EnctyptionError},
    user::graphql::UserUuid,
    utils::{datetime::get_now, uuid::get_uuid_bytes},
};
use entity::{
    enums::Role,
    users::{self, Entity as User},
};
use sea_orm::entity::prelude::Uuid;
use sea_orm::entity::*;
use sea_orm::{DatabaseConnection, DbErr, PaginatorTrait, QueryFilter};
use snafu::prelude::*;

#[derive(Clone, Debug)]
pub struct UserRepositoryInput {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
    pub role: Role,
}

#[derive(Clone, Debug)]
pub struct ChangeRoleInput {
    pub uuid: Uuid,
    pub role: Role,
}

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(conn: &DatabaseConnection, id: Uuid) -> Result<Option<users::Model>> {
        User::find()
            .filter(users::Column::Uuid.eq(id))
            .one(conn)
            .await
            .context(QueryFailedSnafu)
    }

    pub async fn find_by_ids(
        conn: &DatabaseConnection,
        ids: &[UserUuid],
    ) -> Result<Vec<users::Model>> {
        let ids_vec: Vec<Uuid> = ids.iter().map(|id| id.0).collect();

        User::find()
            .filter(users::Column::Uuid.is_in(ids_vec))
            .all(conn)
            .await
            .context(QueryFailedSnafu)
    }

    pub async fn find_users_amount(conn: &DatabaseConnection) -> Result<usize> {
        User::find().count(conn).await.context(QueryFailedSnafu)
    }

    pub async fn create(
        conn: &DatabaseConnection,
        input: UserRepositoryInput,
    ) -> Result<users::Model> {
        if input.password != input.password_confirmation {
            return Err(Error::PasswordMismatch);
        }

        let password = encrypt_password(&input.password).context(EncryptionFailedSnafu)?;

        let user = users::ActiveModel {
            email: Set(input.email),
            password: Set(password),
            role: Set(input.role.to_string()),
            ..Default::default()
        };

        let res = User::insert(user)
            .exec(conn)
            .await
            .context(QueryFailedSnafu)?;

        let last_id = Uuid::from_bytes(get_uuid_bytes(&res.last_insert_id));
        let user = UserRepository::find_by_id(conn, last_id).await?;

        match user {
            Some(u) => Ok(u),
            None => Err(Error::UserNotFound { id: last_id }),
        }
    }

    pub async fn find_by_credentials(
        conn: &DatabaseConnection,
        email: String,
        password: String,
    ) -> Result<users::Model> {
        let user = User::find()
            .filter(users::Column::Email.eq(email))
            .one(conn)
            .await
            .context(QueryFailedSnafu)?;

        let user = match user {
            Some(u) => Ok(u),
            None => Err(Error::InvalidCredentials),
        }?;

        let is_valid = verify_password(&password, &user.password).context(EncryptionFailedSnafu)?;

        if is_valid {
            Ok(user)
        } else {
            Err(Error::BadPassword)
        }
    }

    pub async fn change_role(
        conn: &DatabaseConnection,
        input: ChangeRoleInput,
    ) -> Result<users::Model> {
        let user = UserRepository::find_by_id(conn, input.uuid)
            .await?
            .context(MissingUserSnafu)?;

        let mut user: users::ActiveModel = user.into();
        user.role = Set(input.role.to_string());
        user.updated_at = Set(get_now());

        let user: users::Model = user.update(conn).await.context(QueryFailedSnafu)?;

        Ok(user)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("User Query failed {}", source))]
    QueryFailed { source: DbErr },

    #[snafu(display("Failed to encrypt the password {}", source))]
    EncryptionFailed { source: EnctyptionError },

    #[snafu(display("Passwords do not match"))]
    PasswordMismatch,

    #[snafu(display("User not found with {id}"))]
    UserNotFound { id: Uuid },

    #[snafu(display("User or password are incorrect"))]
    InvalidCredentials,

    #[snafu(display("Bad Password"))]
    BadPassword,

    #[snafu(display("User does not exist"))]
    MissingUser,
}
