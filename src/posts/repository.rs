use crate::utils::uuid::get_uuid_bytes;
use entity::{
    enums::Status,
    posts::{self, Entity as Post},
};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};
use snafu::prelude::*;
use uuid::Uuid;

pub struct NewPostInput {
    pub status: Status,
    pub title: String,
    pub raw: String,
    pub html: String,
    pub created_by: Uuid,
}

pub struct PostsRepository;

impl PostsRepository {
    pub async fn create(conn: &DatabaseConnection, input: NewPostInput) -> Result<posts::Model> {
        let post = posts::ActiveModel {
            status: Set(input.status.to_string()),
            title: Set(input.title),
            raw: Set(input.raw),
            html: Set(input.html),
            created_by: Set(input.created_by.as_bytes().to_vec()),
            ..Default::default()
        };

        let result = Post::insert(post)
            .exec(conn)
            .await
            .context(QueryFailedSnafu)?;

        let last_insert_id = Uuid::from_bytes(get_uuid_bytes(&result.last_insert_id));

        PostsRepository::find_by_id(conn, last_insert_id)
            .await?
            .context(PostNotFoundSnafu {
                uuid: last_insert_id,
            })
    }

    pub async fn find_by_id(conn: &DatabaseConnection, uuid: Uuid) -> Result<Option<posts::Model>> {
        Post::find_by_id(uuid.as_bytes().to_vec())
            .one(conn)
            .await
            .context(QueryFailedSnafu)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Posts Query failed: {}", source))]
    QueryFailed { source: DbErr },

    #[snafu(display("Post with uuid {} not found", uuid))]
    PostNotFound { uuid: Uuid },
}
