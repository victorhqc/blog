use super::ContentType;
use crate::utils::uuid::get_uuid_bytes;
use entity::uploads::{self, Entity as Upload};
use sea_orm::entity::*;
use sea_orm::{DatabaseConnection, DbErr};
use snafu::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UploadsRepositoryInput {
    pub user_uuid: Uuid,
    pub filename: String,
    pub s3_key: String,
    pub content_type: ContentType,
}

pub struct UploadsRepository;

impl UploadsRepository {
    pub async fn create(
        conn: &DatabaseConnection,
        input: UploadsRepositoryInput,
    ) -> Result<uploads::Model> {
        let photo = uploads::ActiveModel {
            filename: Set(input.filename),
            content_type: Set(input.content_type.to_string()),
            s3_key: Set(input.s3_key),
            created_by: Set(input.user_uuid.as_bytes().to_vec()),
            ..Default::default()
        };

        let res = Upload::insert(photo)
            .exec(conn)
            .await
            .context(QueryFailedSnafu)?;

        let last_insert_id = Uuid::from_bytes(get_uuid_bytes(&res.last_insert_id));
        UploadsRepository::find_by_id(conn, last_insert_id)
            .await?
            .context(PhotoNotFoundSnafu { id: last_insert_id })
    }

    pub async fn remove(conn: &DatabaseConnection, upload: uploads::Model) -> Result<()> {
        upload.delete(conn).await.context(QueryFailedSnafu)?;

        Ok(())
    }

    pub async fn find_by_id(
        conn: &DatabaseConnection,
        uuid: Uuid,
    ) -> Result<Option<uploads::Model>> {
        Upload::find_by_id(uuid.as_bytes().to_vec())
            .one(conn)
            .await
            .context(QueryFailedSnafu)
    }

    pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<uploads::Model>> {
        Upload::find().all(conn).await.context(QueryFailedSnafu)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Photo Query failed: {}", source))]
    QueryFailed { source: DbErr },

    #[snafu(display("Photo not found with {id}"))]
    PhotoNotFound { id: Uuid },
}
