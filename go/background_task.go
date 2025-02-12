// Package svix this file is @generated DO NOT EDIT
package svix

import (
	"context"

	"github.com/svix/svix-webhooks/go/models"
)

type BackgroundTask struct {
	client *SvixHttpClient
}

type BackgroundTaskListOptions struct {

	// Filter the response based on the status.
	Status *models.BackgroundTaskStatus

	// Filter the response based on the type.
	Task *models.BackgroundTaskType
	// Limit the number of returned items
	Limit *uint64
	// The iterator returned from a prior invocation
	Iterator *string

	// The sorting order of the returned items
	Order *models.Ordering
}

// List background tasks executed in the past 90 days.
func (backgroundTask *BackgroundTask) List(
	ctx context.Context,
	o *BackgroundTaskListOptions,
) (*models.ListResponseBackgroundTaskOut, error) {
	queryMap := map[string]string{}
	var err error
	if o != nil {
		serializeParamToMap("status", o.Status, queryMap, &err)
		serializeParamToMap("task", o.Task, queryMap, &err)
		serializeParamToMap("limit", o.Limit, queryMap, &err)
		serializeParamToMap("iterator", o.Iterator, queryMap, &err)
		serializeParamToMap("order", o.Order, queryMap, &err)
		if err != nil {
			return nil, err
		}
	}

	return executeRequest[any, models.ListResponseBackgroundTaskOut](
		ctx,
		backgroundTask.client,
		"GET",
		"/api/v1/background-task",
		nil,
		queryMap,
		nil,
		nil,
	)

}

// Get a background task by ID.
func (backgroundTask *BackgroundTask) Get(
	ctx context.Context,
	taskId string,
) (*models.BackgroundTaskOut, error) {
	pathMap := map[string]string{
		"task_id": taskId,
	}

	return executeRequest[any, models.BackgroundTaskOut](
		ctx,
		backgroundTask.client,
		"GET",
		"/api/v1/background-task/{task_id}",
		pathMap,
		nil,
		nil,
		nil,
	)

}
