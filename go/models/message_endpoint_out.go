// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type MessageEndpointOut struct {
	Channels    []string      `json:"channels,omitempty"` // List of message channels this endpoint listens to (omit for all).
	CreatedAt   time.Time     `json:"createdAt"`
	Description string        `json:"description"` // An example endpoint name.
	Disabled    *bool         `json:"disabled,omitempty"`
	FilterTypes []string      `json:"filterTypes,omitempty"`
	Id          string        `json:"id"` // The Endpoint's ID.
	NextAttempt *time.Time    `json:"nextAttempt,omitempty"`
	RateLimit   *uint16       `json:"rateLimit,omitempty"`
	Status      MessageStatus `json:"status"`
	Uid         *string       `json:"uid,omitempty"` // Optional unique identifier for the endpoint.
	UpdatedAt   time.Time     `json:"updatedAt"`
	Url         string        `json:"url"`
	Version     int32         `json:"version"`
}
