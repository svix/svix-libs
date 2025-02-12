// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type TemplateOut struct {
	CreatedAt        time.Time                  `json:"createdAt"`
	Description      string                     `json:"description"`
	FeatureFlag      *string                    `json:"featureFlag,omitempty"`
	FilterTypes      []string                   `json:"filterTypes,omitempty"`
	Id               string                     `json:"id"`
	Instructions     string                     `json:"instructions"`
	InstructionsLink *string                    `json:"instructionsLink,omitempty"`
	Kind             TransformationTemplateKind `json:"kind"`
	Logo             string                     `json:"logo"`
	Name             string                     `json:"name"`
	OrgId            string                     `json:"orgId"`
	Transformation   string                     `json:"transformation"`
	UpdatedAt        time.Time                  `json:"updatedAt"`
}
