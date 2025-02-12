// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/models"
)

type Application struct {
	client *SvixHttpClient
}

type ApplicationListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

type ApplicationCreateOptions struct {
	IdempotencyKey *string
}

// List of all the organization's applications.
func (application *Application) List(
	ctx context.Context,
	o *ApplicationListOptions,
) (*models.ListResponseApplicationOut, error) {
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

	return executeRequest[any, models.ListResponseApplicationOut](
		ctx,
		application.client,
		"GET",
		"/api/v1/app",
		nil,
		queryMap,
		nil,
		nil,
	)

}

// Create a new application.
func (application *Application) Create(
	ctx context.Context,
	applicationIn models.ApplicationIn,
	o *ApplicationCreateOptions,
) (*models.ApplicationOut, error) {
	queryMap := map[string]string{
		"get_if_exists": "false",
	}
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[models.ApplicationIn, models.ApplicationOut](
		ctx,
		application.client,
		"POST",
		"/api/v1/app",
		nil,
		queryMap,
		headerMap,
		&applicationIn,
	)

}

// Get or create a new application.
func (application *Application) GetOrCreate(
	ctx context.Context,
	applicationIn models.ApplicationIn,
	o *ApplicationCreateOptions,
) (*models.ApplicationOut, error) {
	queryMap := map[string]string{
		"get_if_exists": "true",
	}
	headerMap := map[string]string{}

	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[models.ApplicationIn, models.ApplicationOut](
		ctx,
		application.client,
		"POST",
		"/api/v1/app",
		nil,
		queryMap,
		headerMap,
		&applicationIn,
	)
}

// Get an application.
func (application *Application) Get(
	ctx context.Context,
	appId string,
) (*models.ApplicationOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}

	return executeRequest[any, models.ApplicationOut](
		ctx,
		application.client,
		"GET",
		"/api/v1/app/{app_id}",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Update an application.
func (application *Application) Update(
	ctx context.Context,
	appId string,
	applicationIn models.ApplicationIn,
) (*models.ApplicationOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}

	return executeRequest[models.ApplicationIn, models.ApplicationOut](
		ctx,
		application.client,
		"PUT",
		"/api/v1/app/{app_id}",
		pathMap,
		nil,
		nil,
		&applicationIn,
	)

}

// Delete an application.
func (application *Application) Delete(
	ctx context.Context,
	appId string,
) error {
	pathMap := map[string]string{
		"app_id": appId,
	}

	_, err := executeRequest[any, any](
		ctx,
		application.client,
		"DELETE",
		"/api/v1/app/{app_id}",
		pathMap,
		nil,
		nil,
		nil,
	)
	return err

}

// Partially update an application.
func (application *Application) Patch(
	ctx context.Context,
	appId string,
	applicationPatch models.ApplicationPatch,
) (*models.ApplicationOut, error) {
	pathMap := map[string]string{
		"app_id": appId,
	}

	return executeRequest[models.ApplicationPatch, models.ApplicationOut](
		ctx,
		application.client,
		"PATCH",
		"/api/v1/app/{app_id}",
		pathMap,
		nil,
		nil,
		&applicationPatch,
	)

}
