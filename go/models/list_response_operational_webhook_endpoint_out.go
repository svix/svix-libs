// Package svix this file is @generated DO NOT EDIT
package models

type ListResponseOperationalWebhookEndpointOut struct {
	Data         []OperationalWebhookEndpointOut `json:"data"`
	Done         bool                            `json:"done"`
	Iterator     string                          `json:"iterator"`
	PrevIterator *string                         `json:"prevIterator,omitempty"`
}
