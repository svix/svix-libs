// Package svix this file is @generated DO NOT EDIT
package models

type EventTypeIn struct {
	Archived    *bool           `json:"archived,omitempty"`
	Deprecated  *bool           `json:"deprecated,omitempty"`
	Description string          `json:"description"`
	FeatureFlag *string         `json:"featureFlag,omitempty"`
	GroupName   *string         `json:"groupName,omitempty"` // The event type group's name
	Name        string          `json:"name"`                // The event type's name
	Schemas     *map[string]any `json:"schemas,omitempty"`   // The schema for the event type for a specific version as a JSON schema.
}
