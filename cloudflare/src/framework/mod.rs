/*!
This module controls how requests are sent to Cloudflare's API, and how responses are parsed from it.
 */
pub mod async_api;
pub mod auth;
#[cfg(not(target_arch = "wasm32"))]  // There is no blocking implementation for wasm.
pub mod blocking_api;
pub mod endpoint;
pub mod json_utils;
#[cfg(not(target_arch = "wasm32"))]  // The mock contains a blocking implementation.
pub mod mock;

mod environment;
pub use environment::Environment;

use serde::Serialize;

pub trait ApiResult: serde::de::DeserializeOwned + std::fmt::Debug {}

/// Some endpoints return nothing. That's OK.
impl ApiResult for () {}

#[derive(Serialize, Clone, Debug)]
pub enum OrderDirection {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

/// Used as a parameter to API calls that search for a resource (e.g. DNS records).
/// Tells the API whether to return results that match all search requirements or at least one (any).
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SearchMatch {
    /// Match all search requirements
    All,
    /// Match at least one search requirement
    Any,
}
