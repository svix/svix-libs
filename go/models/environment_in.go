// Package svix this file is @generated DO NOT EDIT
package models

type EnvironmentIn struct {
	EventTypes              []EventTypeIn   `json:"eventTypes,omitempty"`
	Settings                *map[string]any `json:"settings,omitempty"`
	TransformationTemplates []TemplateIn    `json:"transformationTemplates,omitempty"`
}
