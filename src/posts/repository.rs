use crate::{
    tags::{Error as TagsError, TagsRepository},
    utils::uuid::get_uuid_bytes,
};
use entity::{
    enums::Status,
    post_tags::{
        ActiveModel as PostTagsActiveModel, Column as PostTagsColumn, Entity as PostTagsEntity,
    },
    posts::{self, Entity as Post},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use snafu::prelude::*;
use uuid::Uuid;

pub struct NewPostInput {
    pub title: String,
    pub raw: String,
    pub html: String,
    pub tags: Vec<String>,
    pub created_by: Uuid,
}

pub struct UpdatePostInput {
    pub uuid: Uuid,
    pub title: String,
    pub raw: String,
    pub html: String,
    pub tags: Vec<String>,
}

pub struct ChangePostStatusInput {
    pub uuid: Uuid,
    pub status: Status,
}

pub struct PostsRepository;

impl PostsRepository {
    pub async fn create(conn: &DatabaseConnection, input: NewPostInput) -> Result<posts::Model> {
        let post = posts::ActiveModel {
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

        let post = PostsRepository::find_by_id(conn, last_insert_id)
            .await?
            .context(PostNotFoundSnafu {
                uuid: last_insert_id,
            })?;

        PostsRepository::register_tags(conn, &post, input.tags).await?;

        Ok(post)
    }

    pub async fn update_post(
        conn: &DatabaseConnection,
        input: UpdatePostInput,
    ) -> Result<posts::Model> {
        let post = PostsRepository::find_by_id(conn, input.uuid)
            .await?
            .context(PostNotFoundSnafu { uuid: input.uuid })?;

        let mut post: posts::ActiveModel = post.into();
        post.title = Set(input.title);
        post.raw = Set(input.raw);
        post.html = Set(input.html);

        let post: posts::Model = post.update(conn).await.context(QueryFailedSnafu)?;

        PostsRepository::register_tags(conn, &post, input.tags).await?;

        Ok(post)
    }

    pub async fn change_post_status(
        conn: &DatabaseConnection,
        input: ChangePostStatusInput,
    ) -> Result<posts::Model> {
        let post = PostsRepository::find_by_id(conn, input.uuid)
            .await?
            .context(PostNotFoundSnafu { uuid: input.uuid })?;

        let mut post: posts::ActiveModel = post.into();
        post.status = Set(input.status.to_string());

        let post: posts::Model = post.update(conn).await.context(QueryFailedSnafu)?;

        Ok(post)
    }

    pub async fn delete(conn: &DatabaseConnection, uuid: Uuid) -> Result<()> {
        Post::delete_by_id(uuid.as_bytes().to_vec())
            .exec(conn)
            .await
            .context(QueryFailedSnafu)?;

        Ok(())
    }

    pub async fn find_by_id(conn: &DatabaseConnection, uuid: Uuid) -> Result<Option<posts::Model>> {
        Post::find_by_id(uuid.as_bytes().to_vec())
            .one(conn)
            .await
            .context(QueryFailedSnafu)
    }

    pub async fn find_all(conn: &DatabaseConnection) -> Result<Vec<posts::Model>> {
        Post::find().all(conn).await.context(QueryFailedSnafu)
    }

    async fn register_tags(
        conn: &DatabaseConnection,
        post: &posts::Model,
        tags: Vec<String>,
    ) -> Result<()> {
        // 1. Unattach the tags that are linked to the post.
        PostTagsEntity::delete_many()
            .filter(PostTagsColumn::PostUuid.eq(post.uuid.clone()))
            .exec(conn)
            .await
            .context(TagsQueryFailedSnafu)?;

        // 2. Find or create the given tags.
        let tags_to_attach = TagsRepository::find_or_create_tags(conn, tags)
            .await
            .context(TagsRepoFailedSnafu)?;

        // 3. Attach the tags to the Post
        let tags_to_attach: Vec<PostTagsActiveModel> = tags_to_attach
            .into_iter()
            .map(|tag| PostTagsActiveModel {
                post_uuid: Set(post.uuid.clone()),
                tag_uuid: Set(tag.uuid),
                ..Default::default()
            })
            .collect();

        PostTagsEntity::insert_many(tags_to_attach)
            .exec(conn)
            .await
            .context(TagsQueryFailedSnafu)?;

        Ok(())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Posts Query failed: {}", source))]
    QueryFailed { source: DbErr },

    #[snafu(display("Tags Query failed: {}", source))]
    TagsQueryFailed { source: DbErr },

    #[snafu(display("Post with uuid {} not found", uuid))]
    PostNotFound { uuid: Uuid },

    #[snafu(display("Failed in PostsRepository: {}", source))]
    TagsRepoFailed { source: TagsError },
}
