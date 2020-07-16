use crate::framework::async_api;
use crate::framework::endpoint::{Endpoint, Method};
use crate::framework::ApiResult;
use async_trait::async_trait;
use surf::http_types::Result;

pub struct MockApiClient {}

// This endpoint does nothing. Designed for use with MockApiClient.
pub struct NoopEndpoint {}

impl Endpoint<NoopResult> for NoopEndpoint {
    fn method(&self) -> Method {
        Method::Get
    }
    fn path(&self) -> String {
        "no/such/path/".to_owned()
    }
}

#[derive(Deserialize, Debug)]
pub struct NoopResult {}
impl ApiResult for NoopResult {}

fn mock_response() -> surf::Error {
    surf::Error {
        error: anyhow::Error::msg("This is a mocked failure response".to_owned()),
        status: surf::http_types::StatusCode::InternalServerError,
    }
}

#[async_trait]
impl async_api::ApiClient for MockApiClient {
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        _endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> Result<ResultType> {
        Err(mock_response())
    }
}
