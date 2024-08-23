/// ## Inferno API Interface
/// ### Features Implemented:
/// 1. Basic API & Orchestrator
/// 2. Use of Ockam for E2E Encryption between models
/// 3. 
/// 

use ockam::{Context, Result, TcpTransport, route, secure_channel, RemoteForwarder};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use crate::logging::init_logging;
use crate::errors::CustomError;

/// # Inferno API
/// 

pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub async fn new() -> Result<Self> {
        let client = Client::new();
        log.info("Client {} initialized", client);

        Ok(ApiClient { client })
    }
}

/// ## API Request
/// Send a request on a secure channel and receive the response
/// throws a custom error where response status is not success
/// Can probably granularize the errors.
pub async fn send_request<T: Serialize>(
    &self,
    ctx: &Context,
    api_url: &str,
    payload: T,
    forwarder: &RemoteForwarder,
) -> Result<Value> {
    let body = serde_json::to_string(&payload)
        .map_err(|e| {
            log.error(CustomError::Unexpected(format!("Failed to serialize payload: {}", e)))
        }?);

    let route = route![forwarder.remote_route().clone(),
        secure_chanell(api_url)];

    let response = self.client
                    .post(api_url)
                    .body(body)
                    .send()
                    .await
                    .map_err(|e| {
                        log.error(CustomError::ApiRequestFailed(format!("Failed to send request: {}", e)))
                    });

    if response.status().is_success() {
        let json: Value = response
                .json()
                .await
                .map_err(|e| {
                    log.error(CustomError::InvalidResponse(format!("Failed to parse response: {}", e)))
                })?;
    log.info("API Initialized Successfully {} : ", json);
    Ok(json)
} else {
    let status  = response.status();
    log.error(&format!("Received unsuccessful response {:?}", status));
    Err(CustomError::ApiRequestFailed(format!("Received unsuccessful response {:?}", status)))
}}

