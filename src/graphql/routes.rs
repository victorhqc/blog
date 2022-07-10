use super::ApiSchema;
use crate::authorization::jwt::{verify_token, JWTError, MaybeToken};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    BatchResponse, Response, ServerError,
};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use rocket::{response::content, State};

#[rocket::get("/graphql")]
pub fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<ApiSchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<ApiSchema>,
    request: GraphQLRequest,
    token: &MaybeToken,
) -> GraphQLResponse {
    handle_request_with_token(schema, request, token).await
}

#[rocket::post(
    "/graphql",
    data = "<request>",
    format = "multipart/form-data",
    rank = 2
)]
pub async fn graphql_request_multipart(
    schema: &State<ApiSchema>,
    request: GraphQLRequest,
    token: &MaybeToken,
) -> GraphQLResponse {
    handle_request_with_token(schema, request, token).await
}

async fn handle_request_with_token(
    schema: &State<ApiSchema>,
    request: GraphQLRequest,
    token: &MaybeToken,
) -> GraphQLResponse {
    let verified = match verify_token(token) {
        Ok(token) => token,
        Err(err) => match err {
            JWTError::TokenExpired => {
                debug!("Token expired, proceeding without it");
                return request.execute(schema).await;
            }
            JWTError::TokenMissing => {
                debug!("Token does not exist, proceeding without it");
                return request.execute(schema).await;
            }
            _err => {
                let response = Response::from_errors(vec![_err.into()]);
                return GraphQLResponse::from(BatchResponse::Single(response));
            }
        },
    };

    debug!("Verified Token, added to request data");
    request.data(verified).execute(schema).await
}

impl From<JWTError> for ServerError {
    fn from(err: JWTError) -> Self {
        ServerError::new(err.to_string(), None)
    }
}
