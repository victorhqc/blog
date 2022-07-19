use crate::{
    aws::s3::{remove_object, upload_object},
    graphql::context::AWSContext,
};
use aws_sdk_s3::Error as S3Error;
use snafu::prelude::*;
use std::{
    fmt::Display,
    fs::File,
    io::{Error as IOError, Read},
};
use uuid::Uuid;

pub async fn upload_to_s3(
    aws: &AWSContext,
    id: impl Display,
    mut file: File,
    filename: &str,
    content_type: Option<String>,
) -> Result<String> {
    let key = Uuid::new_v4();
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data).context(ReadSnafu)?;

    let s3_key = format!("{}/{}", id, key).to_string();

    upload_object(
        &aws.client,
        &aws.bucket_name,
        filename,
        file_data,
        &s3_key,
        content_type,
    )
    .await
    .context(UploadSnafu)?;

    debug!("File uploaded to S3");

    Ok(s3_key)
}

pub async fn remove_from_s3(aws: &AWSContext, s3_key: &str) -> Result<()> {
    remove_object(&aws.client, &aws.bucket_name, s3_key)
        .await
        .context(RemoveSnafu)?;

    debug!("File removed from S3");

    Ok(())
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to read file: {}", source))]
    Read { source: IOError },

    #[snafu(display("Failed to upload file to S3: {}", source))]
    Upload { source: S3Error },

    #[snafu(display("Failed to remove file from S3: {}", source))]
    Remove { source: S3Error },
}
