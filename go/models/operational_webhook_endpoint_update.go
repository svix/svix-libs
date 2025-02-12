// Package svix this file is @generated DO NOT EDIT
package models

type OperationalWebhookEndpointUpdate struct {
	Description *string            `json:"description,omitempty"`
	Disabled    *bool              `json:"disabled,omitempty"`
	FilterTypes []string           `json:"filterTypes,omitempty"`
	Metadata    *map[string]string `json:"metadata,omitempty"`
	RateLimit   *uint16            `json:"rateLimit,omitempty"`
	Uid         *string            `json:"uid,omitempty"` // Optional unique identifier for the endpoint.
	Url         string             `json:"url"`
}
