// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/models"
)

type Integration struct {
	client *SvixHttpClient
}

type IntegrationListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type IntegrationCreateOptions struct {
	IdempotencyKey *string
}

type IntegrationRotateKeyOptions struct {
	IdempotencyKey *string
}

// List the application's integrations.
func (integration *Integration) List(
	ctx context.Context,
	appId string,
	o *IntegrationListOptions,
) (*models.ListResponseIntegrationOut, error) {
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

	return executeRequest[any, models.ListResponseIntegrationOut](
		ctx,
		integration.client,
		"GET",
		"/api/v1/app/{app_id}/integration",
		pathMap,
		queryMap,
		nil,
		nil,
	)

}

// Create an integration.
func (integration *Integration) Create(
	ctx context.Context,
	appId string,
	integrationIn models.IntegrationIn,
	o *IntegrationCreateOptions,
) (*models.IntegrationOut, error) {
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

	return executeRequest[models.IntegrationIn, models.IntegrationOut](
		ctx,
		integration.client,
		"POST",
		"/api/v1/app/{app_id}/integration",
		pathMap,
		nil,
		headerMap,
		&integrationIn,
	)

}

// Get an integration.
func (integration *Integration) Get(
	ctx context.Context,
	appId string,
	integId string,
) (*models.IntegrationOut, error) {
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}

	return executeRequest[any, models.IntegrationOut](
		ctx,
		integration.client,
		"GET",
		"/api/v1/app/{app_id}/integration/{integ_id}",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Update an integration.
func (integration *Integration) Update(
	ctx context.Context,
	appId string,
	integId string,
	integrationUpdate models.IntegrationUpdate,
) (*models.IntegrationOut, error) {
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}

	return executeRequest[models.IntegrationUpdate, models.IntegrationOut](
		ctx,
		integration.client,
		"PUT",
		"/api/v1/app/{app_id}/integration/{integ_id}",
		pathMap,
		nil,
		nil,
		&integrationUpdate,
	)

}

// Delete an integration.
func (integration *Integration) Delete(
	ctx context.Context,
	appId string,
	integId string,
) error {
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}

	_, err := executeRequest[any, any](
		ctx,
		integration.client,
		"DELETE",
		"/api/v1/app/{app_id}/integration/{integ_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err

}

// Get an integration's key.
//
// Deprecated: GetKey is deprecated.
func (integration *Integration) GetKey(
	ctx context.Context,
	appId string,
	integId string,
) (*models.IntegrationKeyOut, error) {
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}

	return executeRequest[any, models.IntegrationKeyOut](
		ctx,
		integration.client,
		"GET",
		"/api/v1/app/{app_id}/integration/{integ_id}/key",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Rotate the integration's key. The previous key will be immediately revoked.
func (integration *Integration) RotateKey(
	ctx context.Context,
	appId string,
	integId string,
	o *IntegrationRotateKeyOptions,
) (*models.IntegrationKeyOut, error) {
	pathMap := map[string]string{
		"app_id":   appId,
		"integ_id": integId,
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[any, models.IntegrationKeyOut](
		ctx,
		integration.client,
		"POST",
		"/api/v1/app/{app_id}/integration/{integ_id}/key/rotate",
		pathMap,
		nil,
		headerMap,
		nil,
	)

}
