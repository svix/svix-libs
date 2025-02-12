// Package svix this file is @generated DO NOT EDIT
package models

type EventTypeUpdate struct {
	Archived    *bool           `json:"archived,omitempty"`
	Deprecated  *bool           `json:"deprecated,omitempty"`
	Description string          `json:"description"`
	FeatureFlag *string         `json:"featureFlag,omitempty"`
	GroupName   *string         `json:"groupName,omitempty"` // The event type group's name
	Schemas     *map[string]any `json:"schemas,omitempty"`   // The schema for the event type for a specific version as a JSON schema.
}
