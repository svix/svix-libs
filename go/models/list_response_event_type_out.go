// Package svix this file is @generated DO NOT EDIT
package models

type ListResponseEventTypeOut struct {
	Data         []EventTypeOut `json:"data"`
	Done         bool           `json:"done"`
	Iterator     *string        `json:"iterator,omitempty"`
	PrevIterator *string        `json:"prevIterator,omitempty"`
}
