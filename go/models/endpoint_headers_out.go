// Package svix this file is @generated DO NOT EDIT
package models

// The value of the headers is returned in the `headers` field.
//
// Sensitive headers that have been redacted are returned in the sensitive field.
type EndpointHeadersOut struct {
	Headers   map[string]string `json:"headers"`
	Sensitive []string          `json:"sensitive"`
}
