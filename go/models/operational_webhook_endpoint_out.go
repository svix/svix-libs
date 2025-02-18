// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type OperationalWebhookEndpointOut struct {
	CreatedAt   time.Time         `json:"createdAt"`
	Description string            `json:"description"` // An example endpoint name.
	Disabled    *bool             `json:"disabled,omitempty"`
	FilterTypes []string          `json:"filterTypes,omitempty"`
	Id          string            `json:"id"` // The Endpoint's ID.
	Metadata    map[string]string `json:"metadata"`
	RateLimit   *uint16           `json:"rateLimit,omitempty"`
	Uid         *string           `json:"uid,omitempty"` // Optional unique identifier for the endpoint.
	UpdatedAt   time.Time         `json:"updatedAt"`
	Url         string            `json:"url"`
}
