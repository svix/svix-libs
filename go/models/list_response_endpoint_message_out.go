// Package svix this file is @generated DO NOT EDIT
package models

type ListResponseEndpointMessageOut struct {
	Data         []EndpointMessageOut `json:"data"`
	Done         bool                 `json:"done"`
	Iterator     *string              `json:"iterator,omitempty"`
	PrevIterator *string              `json:"prevIterator,omitempty"`
}
