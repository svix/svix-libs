/*
 * Svix API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use crate::Configuration;
use crate::models;
use crate::error::Error;
#[allow(unused_imports)]
use crate::models::*;

/// struct for passing parameters to the method [`create_operational_webhook_endpoint`]
#[derive(Clone, Debug)]
pub struct CreateOperationalWebhookEndpointParams {
    pub operational_webhook_endpoint_in: models::OperationalWebhookEndpointIn,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`delete_operational_webhook_endpoint`]
#[derive(Clone, Debug)]
pub struct DeleteOperationalWebhookEndpointParams {
    /// The ep's ID or UID
    pub endpoint_id: String
}

/// struct for passing parameters to the method [`get_operational_webhook_endpoint`]
#[derive(Clone, Debug)]
pub struct GetOperationalWebhookEndpointParams {
    /// The ep's ID or UID
    pub endpoint_id: String
}

/// struct for passing parameters to the method [`get_operational_webhook_endpoint_secret`]
#[derive(Clone, Debug)]
pub struct GetOperationalWebhookEndpointSecretParams {
    /// The ep's ID or UID
    pub endpoint_id: String
}

/// struct for passing parameters to the method [`list_operational_webhook_endpoints`]
#[derive(Clone, Debug)]
pub struct ListOperationalWebhookEndpointsParams {
    /// Limit the number of returned items
    pub limit: Option<i32>,
    /// The iterator returned from a prior invocation
    pub iterator: Option<String>,
    /// The sorting order of the returned items
    pub order: Option<Ordering>
}

/// struct for passing parameters to the method [`rotate_operational_webhook_endpoint_secret`]
#[derive(Clone, Debug)]
pub struct RotateOperationalWebhookEndpointSecretParams {
    /// The ep's ID or UID
    pub endpoint_id: String,
    pub operational_webhook_endpoint_secret_in: models::OperationalWebhookEndpointSecretIn,
    /// The request's idempotency key
    pub idempotency_key: Option<String>
}

/// struct for passing parameters to the method [`update_operational_webhook_endpoint`]
#[derive(Clone, Debug)]
pub struct UpdateOperationalWebhookEndpointParams {
    /// The ep's ID or UID
    pub endpoint_id: String,
    pub operational_webhook_endpoint_update: models::OperationalWebhookEndpointUpdate
}


/// struct for typed errors of method [`create_operational_webhook_endpoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOperationalWebhookEndpointError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_operational_webhook_endpoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteOperationalWebhookEndpointError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_operational_webhook_endpoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOperationalWebhookEndpointError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_operational_webhook_endpoint_secret`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOperationalWebhookEndpointSecretError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_operational_webhook_endpoints`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListOperationalWebhookEndpointsError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rotate_operational_webhook_endpoint_secret`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RotateOperationalWebhookEndpointSecretError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_operational_webhook_endpoint`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateOperationalWebhookEndpointError {
    Status400(models::HttpErrorOut),
    Status401(models::HttpErrorOut),
    Status403(models::HttpErrorOut),
    Status404(models::HttpErrorOut),
    Status409(models::HttpErrorOut),
    Status422(models::HttpValidationError),
    Status429(models::HttpErrorOut),
    UnknownValue(serde_json::Value),
}


/// Create an operational webhook endpoint.
pub async fn create_operational_webhook_endpoint(configuration: &Configuration, params: CreateOperationalWebhookEndpointParams) -> Result<models::OperationalWebhookEndpointOut, Error> {
    // unbox the parameters
    let operational_webhook_endpoint_in = params.operational_webhook_endpoint_in;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/operational-webhook/endpoint")
    ;
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key", param_value.to_string());
    }
    req = req.with_body_param(operational_webhook_endpoint_in);

    req.execute(configuration).await
}

/// Delete an operational webhook endpoint.
pub async fn delete_operational_webhook_endpoint(configuration: &Configuration, params: DeleteOperationalWebhookEndpointParams) -> Result<(), Error> {
    // unbox the parameters
    let endpoint_id = params.endpoint_id;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::DELETE, "/api/v1/operational-webhook/endpoint/{endpoint_id}")
    ;
    req = req.with_path_param("endpoint_id", endpoint_id.to_string());
    req = req.returns_nothing();

    req.execute(configuration).await
}

/// Get an operational webhook endpoint.
pub async fn get_operational_webhook_endpoint(configuration: &Configuration, params: GetOperationalWebhookEndpointParams) -> Result<models::OperationalWebhookEndpointOut, Error> {
    // unbox the parameters
    let endpoint_id = params.endpoint_id;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::GET, "/api/v1/operational-webhook/endpoint/{endpoint_id}")
    ;
    req = req.with_path_param("endpoint_id", endpoint_id.to_string());

    req.execute(configuration).await
}

/// Get an operational webhook endpoint's signing secret.  This is used to verify the authenticity of the webhook. For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
pub async fn get_operational_webhook_endpoint_secret(configuration: &Configuration, params: GetOperationalWebhookEndpointSecretParams) -> Result<models::OperationalWebhookEndpointSecretOut, Error> {
    // unbox the parameters
    let endpoint_id = params.endpoint_id;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::GET, "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret")
    ;
    req = req.with_path_param("endpoint_id", endpoint_id.to_string());

    req.execute(configuration).await
}

/// List operational webhook endpoints.
pub async fn list_operational_webhook_endpoints(configuration: &Configuration, params: ListOperationalWebhookEndpointsParams) -> Result<models::ListResponseOperationalWebhookEndpointOut, Error> {
    // unbox the parameters
    let limit = params.limit;
    let iterator = params.iterator;
    let order = params.order;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::GET, "/api/v1/operational-webhook/endpoint")
    ;
    if let Some(ref s) = limit {
        let query_value = s.to_string();
        req = req.with_query_param("limit", query_value);
    }
    if let Some(ref s) = iterator {
        let query_value = s.to_string();
        req = req.with_query_param("iterator", query_value);
    }
    if let Some(ref s) = order {
        let query_value = s.to_string();
        req = req.with_query_param("order", query_value);
    }

    req.execute(configuration).await
}

/// Rotates an operational webhook endpoint's signing secret.  The previous secret will remain valid for the next 24 hours.
pub async fn rotate_operational_webhook_endpoint_secret(configuration: &Configuration, params: RotateOperationalWebhookEndpointSecretParams) -> Result<(), Error> {
    // unbox the parameters
    let endpoint_id = params.endpoint_id;
    let operational_webhook_endpoint_secret_in = params.operational_webhook_endpoint_secret_in;
    let idempotency_key = params.idempotency_key;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::POST, "/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate")
    ;
    req = req.with_path_param("endpoint_id", endpoint_id.to_string());
    if let Some(param_value) = idempotency_key {
        req = req.with_header_param("idempotency-key", param_value.to_string());
    }
    req = req.with_body_param(operational_webhook_endpoint_secret_in);
    req = req.returns_nothing();

    req.execute(configuration).await
}

/// Update an operational webhook endpoint.
pub async fn update_operational_webhook_endpoint(configuration: &Configuration, params: UpdateOperationalWebhookEndpointParams) -> Result<models::OperationalWebhookEndpointOut, Error> {
    // unbox the parameters
    let endpoint_id = params.endpoint_id;
    let operational_webhook_endpoint_update = params.operational_webhook_endpoint_update;


    #[allow(unused_mut)]
    let mut req = crate::request::Request::new(http1::Method::PUT, "/api/v1/operational-webhook/endpoint/{endpoint_id}")
    ;
    req = req.with_path_param("endpoint_id", endpoint_id.to_string());
    req = req.with_body_param(operational_webhook_endpoint_update);

    req.execute(configuration).await
}

