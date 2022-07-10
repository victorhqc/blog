use chrono::{DateTime, Utc};

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
    }
}

fn get_now() -> DateTime<Utc> {
    Utc::now()
}
