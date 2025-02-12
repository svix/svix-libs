// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/models"
)

type OperationalWebhookEndpoint struct {
	client *SvixHttpClient
}

type OperationalWebhookEndpointListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type OperationalWebhookEndpointCreateOptions struct {
	IdempotencyKey *string
}

type OperationalWebhookEndpointRotateSecretOptions struct {
	IdempotencyKey *string
}

// List operational webhook endpoints.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) List(
	ctx context.Context,
	o *OperationalWebhookEndpointListOptions,
) (*models.ListResponseOperationalWebhookEndpointOut, error) {
	queryMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("limit", o.Limit, queryMap, &err)
		serializeParamToMap("iterator", o.Iterator, queryMap, &err)
		serializeParamToMap("order", o.Order, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[any, models.ListResponseOperationalWebhookEndpointOut](
		ctx,
		operationalWebhookEndpoint.client,
		"GET",
		"/api/v1/operational-webhook/endpoint",
		nil,
		queryMap,
		nil,
		nil,
	)

}

// Create an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Create(
	ctx context.Context,
	operationalWebhookEndpointIn models.OperationalWebhookEndpointIn,
	o *OperationalWebhookEndpointCreateOptions,
) (*models.OperationalWebhookEndpointOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[models.OperationalWebhookEndpointIn, models.OperationalWebhookEndpointOut](
		ctx,
		operationalWebhookEndpoint.client,
		"POST",
		"/api/v1/operational-webhook/endpoint",
		nil,
		nil,
		headerMap,
		&operationalWebhookEndpointIn,
	)

}

// Get an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Get(
	ctx context.Context,
	endpointId string,
) (*models.OperationalWebhookEndpointOut, error) {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}

	return executeRequest[any, models.OperationalWebhookEndpointOut](
		ctx,
		operationalWebhookEndpoint.client,
		"GET",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Update an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Update(
	ctx context.Context,
	endpointId string,
	operationalWebhookEndpointUpdate models.OperationalWebhookEndpointUpdate,
) (*models.OperationalWebhookEndpointOut, error) {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}

	return executeRequest[models.OperationalWebhookEndpointUpdate, models.OperationalWebhookEndpointOut](
		ctx,
		operationalWebhookEndpoint.client,
		"PUT",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}",
		pathMap,
		nil,
		nil,
		&operationalWebhookEndpointUpdate,
	)

}

// Delete an operational webhook endpoint.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) Delete(
	ctx context.Context,
	endpointId string,
) error {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}

	_, err := executeRequest[any, any](
		ctx,
		operationalWebhookEndpoint.client,
		"DELETE",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err

}

// Get the additional headers to be sent with the operational webhook.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) GetHeaders(
	ctx context.Context,
	endpointId string,
) (*models.OperationalWebhookEndpointHeadersOut, error) {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}

	return executeRequest[any, models.OperationalWebhookEndpointHeadersOut](
		ctx,
		operationalWebhookEndpoint.client,
		"GET",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Set the additional headers to be sent with the operational webhook.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) UpdateHeaders(
	ctx context.Context,
	endpointId string,
	operationalWebhookEndpointHeadersIn models.OperationalWebhookEndpointHeadersIn,
) error {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}

	_, err := executeRequest[models.OperationalWebhookEndpointHeadersIn, any](
		ctx,
		operationalWebhookEndpoint.client,
		"PUT",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}/headers",
		pathMap,
		nil,
		nil,
		&operationalWebhookEndpointHeadersIn,
	)
	return err

}

// Get an operational webhook endpoint's signing secret.
//
// This is used to verify the authenticity of the webhook.
// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) GetSecret(
	ctx context.Context,
	endpointId string,
) (*models.OperationalWebhookEndpointSecretOut, error) {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}

	return executeRequest[any, models.OperationalWebhookEndpointSecretOut](
		ctx,
		operationalWebhookEndpoint.client,
		"GET",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}/secret",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Rotates an operational webhook endpoint's signing secret.
//
// The previous secret will remain valid for the next 24 hours.
func (operationalWebhookEndpoint *OperationalWebhookEndpoint) RotateSecret(
	ctx context.Context,
	endpointId string,
	operationalWebhookEndpointSecretIn models.OperationalWebhookEndpointSecretIn,
	o *OperationalWebhookEndpointRotateSecretOptions,
) error {
	pathMap := map[string]string{
		"endpoint_id": endpointId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return err
		}
	}

	_, err = executeRequest[models.OperationalWebhookEndpointSecretIn, any](
		ctx,
		operationalWebhookEndpoint.client,
		"POST",
		"/api/v1/operational-webhook/endpoint/{endpoint_id}/secret/rotate",
		pathMap,
		nil,
		headerMap,
		&operationalWebhookEndpointSecretIn,
	)
	return err

}
