// Package svix this file is @generated DO NOT EDIT
package models

type ReplayOut struct {
	Id     string               `json:"id"`
	Status BackgroundTaskStatus `json:"status"`
	Task   BackgroundTaskType   `json:"task"`
}
