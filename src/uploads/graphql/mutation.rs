use crate::{
    authorization::graphql::{Action, Resource, RoleGuard},
    graphql::context::{get_aws_from_context, get_conn_from_context, get_token_from_context},
    uploads::{
        aws::{remove_from_s3, upload_to_s3},
        graphql::FileUpload,
        repository::{UploadsRepository, UploadsRepositoryInput},
    },
};
use async_graphql::{Context, Object, Result, Upload, ID};
use snafu::prelude::*;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Default)]
pub struct UploadMutation;

#[Object]
impl UploadMutation {
    #[graphql(guard = "RoleGuard::new(Resource::File, Action::Write)")]
    async fn upload_file(&self, ctx: &Context<'_>, file: Upload) -> Result<FileUpload> {
        let aws = get_aws_from_context(ctx).await?;
        let token = get_token_from_context(ctx)
            .await?
            .context(MissingTokenSnafu)?;
        let conn = get_conn_from_context(ctx).await?;

        let upload = file.value(ctx)?;

        let s3_key = upload_to_s3(
            aws,
            token.uuid,
            upload.content,
            &upload.filename,
            upload.content_type.clone(),
        )
        .await?;

        let input = UploadsRepositoryInput {
            user_uuid: token.uuid,
            filename: upload.filename,
            s3_key,
            content_type: upload.content_type.try_into()?,
        };

        let upload = UploadsRepository::create(conn, input).await?;

        Ok(upload.into())
    }

    #[graphql(guard = "RoleGuard::new(Resource::File, Action::Write)")]
    async fn remove_file(&self, ctx: &Context<'_>, uuid: String) -> Result<ID> {
        let aws = get_aws_from_context(ctx).await?;
        let conn = get_conn_from_context(ctx).await?;

        let uuid = Uuid::from_str(&uuid)?;
        let file = UploadsRepository::find_by_id(conn, uuid)
            .await?
            .context(MissingFileSnafu { uuid })?;

        remove_from_s3(aws, &file.s3_key).await?;
        UploadsRepository::remove(conn, file).await?;

        Ok(uuid.into())
    }
}

#[derive(Snafu, Debug)]
pub enum Error {
    #[snafu(display("Token is Missing from Request"))]
    MissingToken,

    #[snafu(display("File does not exist with uuid {}", uuid))]
    MissingFile { uuid: Uuid },
}
