//! This module contains the synchronous (blocking) API client.
use crate::framework::{
    auth::{AuthClient, Credentials},
    endpoint::Endpoint,
    environment::Environment,
    reqwest_adaptors, ApiResult,
};

use anyhow::Result;
use serde::Serialize;
use std::time::Duration;

/// Synchronously sends requests to the Cloudflare API.
pub trait ApiClient {
    /// Synchronously send a request to the Cloudflare API.
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
    ) -> Result<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize;
}

/// Synchronous Cloudflare API client.
pub struct HttpApiClient {
    environment: Environment,
    credentials: Credentials,
    http_client: reqwest::blocking::Client,
}

/// Configuration for the API client. Allows users to customize its behaviour.
pub struct HttpApiClientConfig {
    /// The maximum time limit for an API request. If a request takes longer than this, it will be cancelled.
    pub http_timeout: Duration,
    /// A default set of HTTP headers which will be sent with each API request.
    pub default_headers: http::HeaderMap,
}

impl Default for HttpApiClientConfig {
    fn default() -> Self {
        HttpApiClientConfig {
            http_timeout: Duration::from_secs(30),
            default_headers: http::HeaderMap::default(),
        }
    }
}

impl HttpApiClient {
    pub fn new(
        credentials: Credentials,
        config: HttpApiClientConfig,
        environment: Environment,
    ) -> Result<HttpApiClient> {
        let http_client = reqwest::blocking::Client::builder()
            .timeout(config.http_timeout)
            .default_headers(config.default_headers)
            .build()?;

        Ok(HttpApiClient {
            environment,
            credentials,
            http_client,
        })
    }
}

// TODO: This should probably just implement request for the Reqwest client itself :)
// TODO: It should also probably be called `ReqwestApiClient` rather than `HttpApiClient`.
impl<'a> ApiClient for HttpApiClient {
    /// Synchronously send a request to the Cloudflare API.
    fn request<ResultType, QueryType, BodyType>(
        &self,
        endpoint: &dyn Endpoint<ResultType, QueryType, BodyType>,
    ) -> Result<ResultType>
    where
        ResultType: ApiResult,
        QueryType: Serialize,
        BodyType: Serialize,
    {
        // Build the request
        let mut request = self
            .http_client
            .request(
                reqwest_adaptors::match_reqwest_method(endpoint.method()),
                endpoint.url(&self.environment),
            )
            .query(&endpoint.query());

        if let Some(body) = endpoint.body() {
            request = request.body(serde_json::to_string(&body).unwrap());
            request = request.header(reqwest::header::CONTENT_TYPE, endpoint.content_type());
        }

        request = request.auth(&self.credentials);

        let response = request.send()?;
    }
}
