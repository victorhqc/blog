mod mutation;
mod query;

pub use mutation::*;
pub use query::*;

use async_graphql::{SimpleObject, ID};
use chrono::{DateTime, Utc};
use entity::uploads;
use std::{convert::From, env};
use uuid::Uuid;

#[derive(SimpleObject, Clone)]
pub struct FileUpload {
    pub uuid: ID,
    pub filename: String,
    pub content_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // #[graphql(skip)]
    // created_by: String,
    pub urls: Urls,
}

#[derive(SimpleObject, Clone)]
pub struct Urls {
    url: String,
}

impl From<uploads::Model> for FileUpload {
    fn from(file_upload: uploads::Model) -> Self {
        let uuid = Uuid::from_bytes(file_upload.uuid());

        let api_url = env::var("API_URL").expect("API_URL is not set");
        let urls = Urls {
            url: format!("{}/file/{}", api_url, uuid),
        };

        // let created_by = Uuid::from_bytes(file_upload.created_by());

        FileUpload {
            uuid: uuid.into(),
            filename: file_upload.filename,
            content_type: file_upload.content_type,
            created_at: file_upload.created_at,
            updated_at: file_upload.updated_at,
            urls, // created_by: created_by.to_string(),
        }
    }
}
