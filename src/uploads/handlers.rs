use crate::{
    authorization::cookies::{token_from_cookies, Error as CookiesError},
    aws::s3::download_object,
    db::Db,
    uploads::{repository::Error as UploadRepositoryError, UploadsRepository},
};
use aws_sdk_s3::{Client, Error as S3Error};
use rocket::{
    futures::TryStreamExt,
    http::{hyper::body::Bytes, ContentType, CookieJar, Status},
    request::Request,
    response::{self, stream::ByteStream, Responder, Response},
    serde::{
        ser::{SerializeStruct, Serializer},
        Serialize,
    },
    State,
};
use sea_orm_rocket::Connection;
use snafu::prelude::*;
use std::{
    env::{self, VarError},
    io::Cursor,
    str::FromStr,
};
use uuid::{Error as UUIDError, Uuid};

#[get("/file/<uuid>")]
pub async fn get_file(
    uuid: String,
    client: &State<Client>,
    conn: Connection<'_, Db>,
    cookies: &CookieJar<'_>,
) -> Result<(Status, (ContentType, ByteStream![Bytes]))> {
    let _token = token_from_cookies(cookies).context(InvalidCookieSnafu)?;

    let bucket_name = env::var("AWS_BUCKET_NAME").context(MissingBucketSnafu)?;

    let uuid = Uuid::from_str(&uuid).context(InvalidUuidSnafu)?;

    let upload = UploadsRepository::find_by_id(conn.into_inner(), uuid)
        .await
        .context(FailedToGetFileSnafu)?
        .context(MissingFileSnafu)?;

    let object = download_object(client, &bucket_name, &upload.s3_key)
        .await
        .context(FailedToDownloadFileSnafu)?;

    let content_type = ContentType::from_str(&upload.content_type).unwrap();

    Ok((
        Status::Ok,
        (
            content_type,
            ByteStream! {
                let mut stream = object.body;

                while let Some(bytes) = stream.try_next().await.unwrap() {
                    yield bytes
                }
            },
        ),
    ))
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("AWS_BUCKET_NAME is not defined"))]
    MissingBucket { source: VarError },

    #[snafu(display("Invalid UUID: {}", source))]
    InvalidUuid { source: UUIDError },

    #[snafu(display("Failed to get file"))]
    FailedToGetFile { source: UploadRepositoryError },

    #[snafu(display("File does not exist"))]
    MissingFile,

    #[snafu(display("Failed to download the file"))]
    FailedToDownloadFile { source: S3Error },

    #[snafu(display("{}", source))]
    InvalidCookie { source: CookiesError },
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status: Status = match &self {
            Error::MissingBucket { source: _ } => Status::InternalServerError,
            Error::InvalidUuid { source: _ } => Status::BadRequest,
            Error::FailedToGetFile { source: err } => match err {
                UploadRepositoryError::FileNotFound { id: _ } => Status::NotFound,
                _ => Status::InternalServerError,
            },
            Error::MissingFile => Status::NotFound,
            Error::FailedToDownloadFile { source: _ } => Status::InternalServerError,
            Error::InvalidCookie { source: _ } => Status::Unauthorized,
        };

        let serialized = serde_json::to_string(&self).unwrap();

        Response::build()
            .status(status)
            .sized_body(serialized.len(), Cursor::new(serialized))
            .header(ContentType::JSON)
            .ok()
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let kind: String = match self {
            Error::MissingBucket { source: _ } => String::from("MissingBucket"),
            Error::InvalidUuid { source: _ } => String::from("InvalidUuid"),
            Error::FailedToGetFile { source: err } => {
                debug!("Failed to get file {:?}", err);

                String::from("FailedToGetFile")
            }
            Error::MissingFile => String::from("MissingFile"),
            Error::FailedToDownloadFile { source: err } => {
                debug!("Failed to download photo from S3: {:?}", err);

                String::from("FailedToDownloadFile")
            }
            Error::InvalidCookie { source: err } => {
                debug!("Invalid cookie: {:?}", err);

                String::from("InvalidCookie")
            }
        };

        let mut state = serializer.serialize_struct("Error", 2)?;
        state.serialize_field("kind", &kind)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}
