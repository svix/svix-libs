// Package svix this file is @generated DO NOT EDIT
package models

type ListResponseMessageAttemptOut struct {
	Data         []MessageAttemptOut `json:"data"`
	Done         bool                `json:"done"`
	Iterator     string              `json:"iterator"`
	PrevIterator *string             `json:"prevIterator,omitempty"`
}
