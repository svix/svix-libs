// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"
	"time"

	"github.com/svix/svix-webhooks/go/models"
)

type Endpoint struct {
	client *SvixHttpClient
}

type EndpointListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type EndpointCreateOptions struct {
	IdempotencyKey *string
}

type EndpointRecoverOptions struct {
	IdempotencyKey *string
}

type EndpointReplayMissingOptions struct {
	IdempotencyKey *string
}

type EndpointRotateSecretOptions struct {
	IdempotencyKey *string
}

type EndpointSendExampleOptions struct {
	IdempotencyKey *string
}

type EndpointGetStatsOptions struct {
	// Filter the range to data starting from this date.
	Since *time.Time
	// Filter the range to data ending by this date.
	Until *time.Time
}

// List the application's endpoints.
func (endpoint *Endpoint) List(
	ctx context.Context,
	appId string,
	o *EndpointListOptions,
) (*models.ListResponseEndpointOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
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

	return executeRequest[any, models.ListResponseEndpointOut](
		ctx,
		endpoint.client,
		"GET",
		"/api/v1/app/{app_id}/endpoint",
		pathMap,
		queryMap,
		nil,
		nil,
	)

}

// Create a new endpoint for the application.
//
// When `secret` is `null` the secret is automatically generated (recommended).
func (endpoint *Endpoint) Create(
	ctx context.Context,
	appId string,
	endpointIn models.EndpointIn,
	o *EndpointCreateOptions,
) (*models.EndpointOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[models.EndpointIn, models.EndpointOut](
		ctx,
		endpoint.client,
		"POST",
		"/api/v1/app/{app_id}/endpoint",
		pathMap,
		nil,
		headerMap,
		&endpointIn,
	)

}

// Get an endpoint.
func (endpoint *Endpoint) Get(
	ctx context.Context,
	appId string,
	endpointId string,
) (*models.EndpointOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}

	return executeRequest[any, models.EndpointOut](
		ctx,
		endpoint.client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Update an endpoint.
func (endpoint *Endpoint) Update(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointUpdate models.EndpointUpdate,
) (*models.EndpointOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}

	return executeRequest[models.EndpointUpdate, models.EndpointOut](
		ctx,
		endpoint.client,
		"PUT",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}",
		pathMap,
		nil,
		nil,
		&endpointUpdate,
	)

}

// Delete an endpoint.
func (endpoint *Endpoint) Delete(
	ctx context.Context,
	appId string,
	endpointId string,
) error {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}

	_, err := executeRequest[any, any](
		ctx,
		endpoint.client,
		"DELETE",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err

}

// Partially update an endpoint.
func (endpoint *Endpoint) Patch(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointPatch models.EndpointPatch,
) (*models.EndpointOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}

	return executeRequest[models.EndpointPatch, models.EndpointOut](
		ctx,
		endpoint.client,
		"PATCH",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}",
		pathMap,
		nil,
		nil,
		&endpointPatch,
	)

}

// Get the additional headers to be sent with the webhook.
func (endpoint *Endpoint) GetHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
) (*models.EndpointHeadersOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}

	return executeRequest[any, models.EndpointHeadersOut](
		ctx,
		endpoint.client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Set the additional headers to be sent with the webhook.
func (endpoint *Endpoint) UpdateHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointHeadersIn models.EndpointHeadersIn,
) error {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}

	_, err := executeRequest[models.EndpointHeadersIn, any](
		ctx,
		endpoint.client,
		"PUT",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
		pathMap,
		nil,
		nil,
		&endpointHeadersIn,
	)
	return err

}

// Partially set the additional headers to be sent with the webhook.
func (endpoint *Endpoint) PatchHeaders(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointHeadersPatchIn models.EndpointHeadersPatchIn,
) error {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}

	_, err := executeRequest[models.EndpointHeadersPatchIn, any](
		ctx,
		endpoint.client,
		"PATCH",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/headers",
		pathMap,
		nil,
		nil,
		&endpointHeadersPatchIn,
	)
	return err

}

// Resend all failed messages since a given time.
//
// Messages that were sent successfully, even if failed initially, are not resent.
func (endpoint *Endpoint) Recover(
	ctx context.Context,
	appId string,
	endpointId string,
	recoverIn models.RecoverIn,
	o *EndpointRecoverOptions,
) (*models.RecoverOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[models.RecoverIn, models.RecoverOut](
		ctx,
		endpoint.client,
		"POST",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/recover",
		pathMap,
		nil,
		headerMap,
		&recoverIn,
	)

}

// Replays messages to the endpoint.
//
// Only messages that were created after `since` will be sent.
// Messages that were previously sent to the endpoint are not resent.
func (endpoint *Endpoint) ReplayMissing(
	ctx context.Context,
	appId string,
	endpointId string,
	replayIn models.ReplayIn,
	o *EndpointReplayMissingOptions,
) (*models.ReplayOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[models.ReplayIn, models.ReplayOut](
		ctx,
		endpoint.client,
		"POST",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/replay-missing",
		pathMap,
		nil,
		headerMap,
		&replayIn,
	)

}

// Get the endpoint's signing secret.
//
// This is used to verify the authenticity of the webhook.
// For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
func (endpoint *Endpoint) GetSecret(
	ctx context.Context,
	appId string,
	endpointId string,
) (*models.EndpointSecretOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}

	return executeRequest[any, models.EndpointSecretOut](
		ctx,
		endpoint.client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Rotates the endpoint's signing secret.
//
// The previous secret will remain valid for the next 24 hours.
func (endpoint *Endpoint) RotateSecret(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointSecretRotateIn models.EndpointSecretRotateIn,
	o *EndpointRotateSecretOptions,
) error {
	pathMap := map[string]string{
		"app_id":      appId,
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

	_, err = executeRequest[models.EndpointSecretRotateIn, any](
		ctx,
		endpoint.client,
		"POST",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/secret/rotate",
		pathMap,
		nil,
		headerMap,
		&endpointSecretRotateIn,
	)
	return err

}

// Send an example message for an event.
func (endpoint *Endpoint) SendExample(
	ctx context.Context,
	appId string,
	endpointId string,
	eventExampleIn models.EventExampleIn,
	o *EndpointSendExampleOptions,
) (*models.MessageOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[models.EventExampleIn, models.MessageOut](
		ctx,
		endpoint.client,
		"POST",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/send-example",
		pathMap,
		nil,
		headerMap,
		&eventExampleIn,
	)

}

// Get basic statistics for the endpoint.
func (endpoint *Endpoint) GetStats(
	ctx context.Context,
	appId string,
	endpointId string,
	o *EndpointGetStatsOptions,
) (*models.EndpointStats, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("since", o.Since, queryMap, &err)
		serializeParamToMap("until", o.Until, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[any, models.EndpointStats](
		ctx,
		endpoint.client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/stats",
		pathMap,
		queryMap,
		nil,
		nil,
	)

}

// Get the transformation code associated with this endpoint.
func (endpoint *Endpoint) TransformationGet(
	ctx context.Context,
	appId string,
	endpointId string,
) (*models.EndpointTransformationOut, error) {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}

	return executeRequest[any, models.EndpointTransformationOut](
		ctx,
		endpoint.client,
		"GET",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Set or unset the transformation code associated with this endpoint.
func (endpoint *Endpoint) TransformationPartialUpdate(
	ctx context.Context,
	appId string,
	endpointId string,
	endpointTransformationIn models.EndpointTransformationIn,
) error {
	pathMap := map[string]string{
		"app_id":      appId,
		"endpoint_id": endpointId,
	}

	_, err := executeRequest[models.EndpointTransformationIn, any](
		ctx,
		endpoint.client,
		"PATCH",
		"/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation",
		pathMap,
		nil,
		nil,
		&endpointTransformationIn,
	)
	return err

}
