// Package svix this file is @generated DO NOT EDIT
package models

type ListResponseEndpointOut struct {
	Data         []EndpointOut `json:"data"`
	Done         bool          `json:"done"`
	Iterator     string        `json:"iterator"`
	PrevIterator *string       `json:"prevIterator,omitempty"`
}
