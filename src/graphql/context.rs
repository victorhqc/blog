use crate::{authorization::jwt::AuthorizedToken, aws::build_client};
use async_graphql::{Context, Error as AsyncGraphQLError};
use async_mutex::{Mutex, MutexGuard};
use aws_sdk_s3::{Client, Error as S3Error};
use casbin::Enforcer;
use sea_orm::DatabaseConnection;
use snafu::prelude::*;
use std::{env, sync::Arc};

#[derive(Clone)]
pub struct AppContext {
    pub enforcer: Arc<Mutex<Enforcer>>,
    pub aws: AWSContext,
}

#[derive(Clone)]
pub struct AWSContext {
    pub client: Client,
    pub bucket_name: String,
}

impl AWSContext {
    pub async fn default() -> Result<Self, AWSContextError> {
        let bucket_name = env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME is not defined");
        let client = build_client().await.context(ClientBuildIssueSnafu)?;

        Ok(AWSContext {
            client,
            bucket_name,
        })
    }
}

#[derive(Debug, Snafu)]
pub enum AWSContextError {
    #[snafu(display("Can't build the AWS Client: {:?}", source))]
    ClientBuildIssue { source: S3Error },
}

pub async fn get_enforcer_from_context<'a>(
    ctx: &'a Context<'_>,
) -> Result<MutexGuard<'a, Enforcer>, AsyncGraphQLError> {
    let AppContext { enforcer, .. } = ctx.data()?;

    let enforcer = enforcer.lock().await;

    Ok(enforcer)
}

pub async fn get_aws_from_context<'a>(
    ctx: &'a Context<'_>,
) -> Result<&'a AWSContext, AsyncGraphQLError> {
    let AppContext { aws, .. } = ctx.data()?;
    Ok(aws)
}

pub async fn get_token_from_context<'a>(
    ctx: &'a Context<'_>,
) -> Result<Option<&'a AuthorizedToken>, AsyncGraphQLError> {
    let token = ctx.data_opt::<AuthorizedToken>();
    Ok(token)
}

pub async fn get_conn_from_context<'a>(
    ctx: &'a Context<'_>,
) -> Result<&'a DatabaseConnection, AsyncGraphQLError> {
    let conn = ctx.data_opt::<DatabaseConnection>();
    let conn = match conn {
        Some(c) => c,
        None => {
            return Err(AsyncGraphQLError::from(ConnError::ConnectionMissing));
        }
    };

    Ok(conn)
}

#[derive(Snafu, Debug)]
pub enum ConnError {
    #[snafu(display("DB Connection is not in Context"))]
    ConnectionMissing,
}
