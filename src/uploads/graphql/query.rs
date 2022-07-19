use super::FileUpload;
use crate::{
    authorization::graphql::{Action, Resource, RoleGuard},
    graphql::context::get_conn_from_context,
    uploads::UploadsRepository,
};
use async_graphql::{Context, Object, Result};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Default)]
pub struct UploadQuery;

#[Object]
impl UploadQuery {
    #[graphql(guard = "RoleGuard::new(Resource::File, Action::Read)")]
    pub async fn file(&self, ctx: &Context<'_>, uuid: String) -> Result<Option<FileUpload>> {
        let conn = get_conn_from_context(ctx).await?;
        let uuid = Uuid::from_str(&uuid)?;

        let file = UploadsRepository::find_by_id(conn, uuid).await?;

        match file {
            Some(f) => Ok(Some(f.into())),
            None => Ok(None),
        }
    }

    pub async fn all_files(&self, ctx: &Context<'_>) -> Result<Vec<FileUpload>> {
        let conn = get_conn_from_context(ctx).await?;

        let files = UploadsRepository::find_all(conn)
            .await?
            .into_iter()
            .map(|f| f.into())
            .collect();

        Ok(files)
    }
}
