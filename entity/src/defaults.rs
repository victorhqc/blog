use crate::{
    post_tags::Relation as PostTagsRelation, posts::Entity as PostEntity, tags::Entity as TagEntity,
};
use chrono::{DateTime, Utc};
use sea_orm::{Linked, RelationDef, RelationTrait};

#[derive(Debug)]
pub struct TagsForPost;

impl Linked for TagsForPost {
    type FromEntity = PostEntity;
    type ToEntity = TagEntity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            PostTagsRelation::Posts.def().rev(),
            PostTagsRelation::Tags.def(),
        ]
    }
}

#[derive(Debug)]
pub struct PostsForTag;

impl Linked for PostsForTag {
    type FromEntity = TagEntity;
    type ToEntity = PostEntity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            PostTagsRelation::Tags.def().rev(),
            PostTagsRelation::Posts.def(),
        ]
    }
}

mod users {
    use crate::{
        enums::Role,
        users::{ActiveModel, Model},
    };
    use sea_orm::entity::prelude::*;
    use sea_orm::ActiveValue::Set;
    use uuid::Uuid;

    impl ActiveModelBehavior for ActiveModel {
        fn new() -> Self {
            let now = super::get_now();
            let uuid = Uuid::new_v4().as_bytes().to_vec();

            Self {
                uuid: Set(uuid),
                created_at: Set(now),
                updated_at: Set(now),
                ..ActiveModelTrait::default()
            }
        }
    }

    impl Model {
        pub fn default(uuid: Uuid) -> Self {
            let now = super::get_now();

            Model {
                uuid: uuid.as_bytes().to_vec(),
                email: String::from(""),
                password: String::from(""),
                role: Role::Writer.to_string(),
                created_at: now,
                updated_at: now,
            }
        }

        pub fn uuid(&self) -> [u8; 16] {
            super::get_uuid_bytes(&self.uuid)
        }
    }
}

mod posts {
    use crate::{
        enums::Status,
        posts::{ActiveModel, Model},
    };
    use sea_orm::entity::prelude::*;
    use sea_orm::ActiveValue::Set;
    use uuid::Uuid;

    impl ActiveModelBehavior for ActiveModel {
        fn new() -> Self {
            let now = super::get_now();
            let uuid = Uuid::new_v4().as_bytes().to_vec();

            Self {
                uuid: Set(uuid),
                status: Set(Status::Draft.to_string()),
                created_at: Set(now),
                updated_at: Set(now),
                ..ActiveModelTrait::default()
            }
        }
    }

    impl Model {
        pub fn default(uuid: Uuid) -> Self {
            let now = super::get_now();

            Model {
                uuid: uuid.as_bytes().to_vec(),
                created_by: Uuid::new_v4().as_bytes().to_vec(),
                status: Status::Draft.to_string(),
                title: String::from(""),
                html: String::from(""),
                raw: String::from(""),
                created_at: now,
                updated_at: now,
            }
        }

        pub fn uuid(&self) -> [u8; 16] {
            super::get_uuid_bytes(&self.uuid)
        }
    }
}

mod tags {
    use crate::tags::{ActiveModel, Model};
    use sea_orm::entity::prelude::*;
    use sea_orm::ActiveValue::Set;
    use uuid::Uuid;

    impl ActiveModelBehavior for ActiveModel {
        fn new() -> Self {
            let now = super::get_now();
            let uuid = Uuid::new_v4().as_bytes().to_vec();

            Self {
                uuid: Set(uuid),
                created_at: Set(now),
                updated_at: Set(now),
                ..ActiveModelTrait::default()
            }
        }
    }

    impl Model {
        pub fn default(uuid: Uuid) -> Self {
            let now = super::get_now();

            Model {
                uuid: uuid.as_bytes().to_vec(),
                name: String::from(""),
                created_at: now,
                updated_at: now,
            }
        }

        pub fn uuid(&self) -> [u8; 16] {
            super::get_uuid_bytes(&self.uuid)
        }
    }
}

mod post_tags {
    use crate::post_tags::{ActiveModel, Model};
    use sea_orm::entity::prelude::*;
    use sea_orm::ActiveValue::Set;
    use uuid::Uuid;

    impl ActiveModelBehavior for ActiveModel {
        fn new() -> Self {
            let uuid = Uuid::new_v4().as_bytes().to_vec();

            Self {
                uuid: Set(uuid),
                ..ActiveModelTrait::default()
            }
        }
    }

    impl Model {
        pub fn default(uuid: Uuid) -> Self {
            Model {
                uuid: uuid.as_bytes().to_vec(),
                post_uuid: Uuid::new_v4().as_bytes().to_vec(),
                tag_uuid: Uuid::new_v4().as_bytes().to_vec(),
            }
        }

        pub fn uuid(&self) -> [u8; 16] {
            super::get_uuid_bytes(&self.uuid)
        }
    }
}

mod uploads {
    use crate::uploads::{ActiveModel, Model};
    use sea_orm::entity::prelude::*;
    use sea_orm::ActiveValue::Set;
    use uuid::Uuid;

    impl ActiveModelBehavior for ActiveModel {
        fn new() -> Self {
            let uuid = Uuid::new_v4().as_bytes().to_vec();

            Self {
                uuid: Set(uuid),
                ..ActiveModelTrait::default()
            }
        }
    }

    impl Model {
        pub fn default(uuid: Uuid) -> Self {
            let now = super::get_now();

            Model {
                uuid: uuid.as_bytes().to_vec(),
                content_type: String::from(""),
                filename: String::from(""),
                s3_key: String::from(""),
                created_by: Uuid::new_v4().as_bytes().to_vec(),
                created_at: now,
                updated_at: now,
            }
        }

        pub fn uuid(&self) -> [u8; 16] {
            super::get_uuid_bytes(&self.uuid)
        }

        pub fn created_by(&self) -> [u8; 16] {
            super::get_uuid_bytes(&self.created_by)
        }
    }
}

fn get_now() -> DateTime<Utc> {
    Utc::now()
}

fn get_uuid_bytes(id: &[u8]) -> [u8; 16] {
    let mut uuid: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for (index, byte) in id.iter().enumerate() {
        uuid[index] = *byte;
    }

    uuid
}
