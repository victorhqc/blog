use crate::{tags::graphql::PostTagUuid, utils::vec::vec_diff};
use entity::{
    post_tags::Relation as PostTagsRelation,
    posts::{Column as PostColumn, Entity as PostEntity, Model as Post},
    tags::{self, Entity as Tag},
};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, JoinType, QueryFilter, QuerySelect,
    RelationTrait, Set,
};
use snafu::prelude::*;
use uuid::Uuid;

pub struct TagsRepository;

impl TagsRepository {
    /// Used for the tags dataloader. It returns all the tags grouped by post.
    pub async fn find_by_post_ids(
        conn: &DatabaseConnection,
        ids: &[PostTagUuid],
    ) -> Result<Vec<(Uuid, Vec<tags::Model>)>> {
        let ids_vec: Vec<Uuid> = ids.iter().map(|id| id.0).collect();

        let post_tags: Vec<(tags::Model, Option<Post>)> = tags::Entity::find()
            .select_also(PostEntity)
            .join(JoinType::InnerJoin, PostTagsRelation::Tags.def().rev())
            .join(JoinType::InnerJoin, PostTagsRelation::Posts.def())
            .filter(PostColumn::Uuid.is_in(ids_vec))
            .all(conn)
            .await
            .context(QueryFailedSnafu)?;

        let folded: Vec<(Uuid, Vec<tags::Model>)> = vec![];

        let post_tags = post_tags
            .into_iter()
            .filter_map(|(tag, post)| match post {
                Some(p) => Some((Uuid::from_bytes(p.uuid()), tag)),
                None => None,
            })
            .fold(folded, |mut acc, (post_uuid, tag)| {
                let index = acc.iter().position(|(uuid, _)| uuid == &post_uuid);

                match index {
                    Some(i) => {
                        acc[i].1.push(tag);
                    }
                    None => {
                        acc.push((post_uuid, vec![tag]));
                    }
                };

                acc
            });

        Ok(post_tags)
    }

    pub async fn find_or_create_tags(
        conn: &DatabaseConnection,
        tags: Vec<String>,
    ) -> Result<Vec<tags::Model>> {
        // 1. Check wether the given tags need to be created or not to the DB
        let tags_ref: Vec<&str> = tags.iter().map(|t| t.as_str()).collect();
        let existing_tags = Tag::find()
            .filter(tags::Column::Name.is_in(tags_ref.clone()))
            .all(conn)
            .await
            .context(QueryFailedSnafu)?;

        let existing_tag_names: Vec<String> =
            existing_tags.iter().map(|tag| tag.name.clone()).collect();

        // 2.The tags that do not exist yet will be created.
        let tags_to_create = vec_diff(tags.clone(), existing_tag_names);

        // If there are no tags to create, it means we can skip the create part.
        if tags_to_create.len() == 0 {
            return Ok(existing_tags);
        }

        // 3. Insert the missing tags to the DB
        let new_tags: Vec<tags::ActiveModel> = tags_to_create
            .into_iter()
            .map(|t| tags::ActiveModel {
                name: Set(t),
                ..Default::default()
            })
            .collect();

        Tag::insert_many(new_tags)
            .exec(conn)
            .await
            .context(QueryFailedSnafu)?;

        // 4. Query for all the tags again, they now must exist in the DB
        Tag::find()
            .filter(tags::Column::Name.is_in(tags_ref))
            .all(conn)
            .await
            .context(QueryFailedSnafu)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Tags Query failed: {}", source))]
    QueryFailed { source: DbErr },
}
