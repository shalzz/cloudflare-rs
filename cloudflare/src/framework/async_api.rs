use crate::framework::{
    apiclient::HttpApiClientConfig,
    auth,
    auth::{AuthClient, Credentials},
    endpoint::{Endpoint, Method},
    ApiResult, Environment,
};
use anyhow::Result;
use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub trait ApiClient {
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> Result<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;
}

/// A Cloudflare API client that makes requests asynchronously.
pub struct Client {
    environment: Environment,
    credentials: auth::Credentials,
}

impl AuthClient for reqwest::RequestBuilder {
    fn auth(mut self, credentials: &Credentials) -> Self {
        for (k, v) in credentials.headers() {
            self = self.header(k, v);
        }
        self
    }
}

impl Client {
    pub fn new(
        credentials: auth::Credentials,
        config: HttpApiClientConfig,
        environment: Environment,
    ) -> Result<Client> {
        Ok(Client {
            environment,
            credentials,
        })
    }
}

#[async_trait]
impl ApiClient for Client {
    async fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &(dyn Endpoint<ResultType, QueryType, BodyType> + Send + Sync),
    ) -> Result<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        let mut request = surf::Request::new(
            match_method(endpoint.method()),
            endpoint.url(&self.environment),
        );
        //                .query(&endpoint.query())

        if let Some(body) = endpoint.body() {
            request = request.body_json(&body)?;
            request = request.set_header(reqwest::header::CONTENT_TYPE, endpoint.content_type());
        }

        request = request.auth(&self.credentials);
        request.recv_json().await?
    }
}

pub fn match_method(method: Method) -> surf::http_types::Method {
    match method {
        Method::Get => surf::http_types::Method::Get,
        Method::Post => surf::http_types::Method::Post,
        Method::Delete => surf::http_types::Method::Delete,
        Method::Put => surf::http_types::Method::Put,
        Method::Patch => surf::http_types::Method::Patch,
    }
}
