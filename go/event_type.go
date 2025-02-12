// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/models"
)

type EventType struct {
	client *SvixHttpClient
}

type EventTypeListOptions struct {
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
	// When `true` archived (deleted but not expunged) items are included in the response.
	IncludeArchived *bool
	// When `true` the full item (including the schema) is included in the response.
	WithContent *bool
}

type EventTypeCreateOptions struct {
	IdempotencyKey *string
}

type EventTypeImportOpenapiOptions struct {
	IdempotencyKey *string
}

type EventTypeDeleteOptions struct {
	// By default event types are archived when "deleted". Passing this to `true` deletes them entirely.
	Expunge *bool
}

// Return the list of event types.
func (eventType *EventType) List(
	ctx context.Context,
	o *EventTypeListOptions,
) (*models.ListResponseEventTypeOut, error) {
	queryMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("limit", o.Limit, queryMap, &err)
		serializeParamToMap("iterator", o.Iterator, queryMap, &err)
		serializeParamToMap("order", o.Order, queryMap, &err)
		serializeParamToMap("include_archived", o.IncludeArchived, queryMap, &err)
		serializeParamToMap("with_content", o.WithContent, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[any, models.ListResponseEventTypeOut](
		ctx,
		eventType.client,
		"GET",
		"/api/v1/event-type",
		nil,
		queryMap,
		nil,
		nil,
	)

}

// Create new or unarchive existing event type.
//
// Unarchiving an event type will allow endpoints to filter on it and messages to be sent with it.
// Endpoints filtering on the event type before archival will continue to filter on it.
// This operation does not preserve the description and schemas.
func (eventType *EventType) Create(
	ctx context.Context,
	eventTypeIn models.EventTypeIn,
	o *EventTypeCreateOptions,
) (*models.EventTypeOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[models.EventTypeIn, models.EventTypeOut](
		ctx,
		eventType.client,
		"POST",
		"/api/v1/event-type",
		nil,
		nil,
		headerMap,
		&eventTypeIn,
	)

}

// Given an OpenAPI spec, create new or update existing event types.
// If an existing `archived` event type is updated, it will be unarchived.
//
// The importer will convert all webhooks found in the either the `webhooks` or `x-webhooks`
// top-level.
func (eventType *EventType) ImportOpenapi(
	ctx context.Context,
	eventTypeImportOpenApiIn models.EventTypeImportOpenApiIn,
	o *EventTypeImportOpenapiOptions,
) (*models.EventTypeImportOpenApiOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[models.EventTypeImportOpenApiIn, models.EventTypeImportOpenApiOut](
		ctx,
		eventType.client,
		"POST",
		"/api/v1/event-type/import/openapi",
		nil,
		nil,
		headerMap,
		&eventTypeImportOpenApiIn,
	)

}

// Get an event type.
func (eventType *EventType) Get(
	ctx context.Context,
	eventTypeName string,
) (*models.EventTypeOut, error) {
	pathMap := map[string]string{
		"event_type_name": eventTypeName,
	}

	return executeRequest[any, models.EventTypeOut](
		ctx,
		eventType.client,
		"GET",
		"/api/v1/event-type/{event_type_name}",
		pathMap,
		nil,
		nil,
		nil,
	)

}

// Update an event type.
func (eventType *EventType) Update(
	ctx context.Context,
	eventTypeName string,
	eventTypeUpdate models.EventTypeUpdate,
) (*models.EventTypeOut, error) {
	pathMap := map[string]string{
		"event_type_name": eventTypeName,
	}

	return executeRequest[models.EventTypeUpdate, models.EventTypeOut](
		ctx,
		eventType.client,
		"PUT",
		"/api/v1/event-type/{event_type_name}",
		pathMap,
		nil,
		nil,
		&eventTypeUpdate,
	)

}

// Archive an event type.
//
// Endpoints already configured to filter on an event type will continue to do so after archival.
// However, new messages can not be sent with it and endpoints can not filter on it.
// An event type can be unarchived with the
// [create operation](#operation/create_event_type_api_v1_event_type__post).
func (eventType *EventType) Delete(
	ctx context.Context,
	eventTypeName string,
	o *EventTypeDeleteOptions,
) error {
	pathMap := map[string]string{
		"event_type_name": eventTypeName,
	}
	queryMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("expunge", o.Expunge, queryMap, &err)
		if err != nil {
			return err
		}
	}

	_, err = executeRequest[any, any](
		ctx,
		eventType.client,
		"DELETE",
		"/api/v1/event-type/{event_type_name}",
		pathMap,
		queryMap,
		nil,
		nil,
	)
	return err

}

// Partially update an event type.
func (eventType *EventType) Patch(
	ctx context.Context,
	eventTypeName string,
	eventTypePatch models.EventTypePatch,
) (*models.EventTypeOut, error) {
	pathMap := map[string]string{
		"event_type_name": eventTypeName,
	}

	return executeRequest[models.EventTypePatch, models.EventTypeOut](
		ctx,
		eventType.client,
		"PATCH",
		"/api/v1/event-type/{event_type_name}",
		pathMap,
		nil,
		nil,
		&eventTypePatch,
	)

}
