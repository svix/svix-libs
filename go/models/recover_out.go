// Package svix this file is @generated DO NOT EDIT
package models

type RecoverOut struct {
	Id     string               `json:"id"` // The QueueBackgroundTask's ID.
	Status BackgroundTaskStatus `json:"status"`
	Task   BackgroundTaskType   `json:"task"`
}
