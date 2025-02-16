// Package svix this file is @generated DO NOT EDIT
package models

import "time"

// A model containing information on a given message plus additional fields on the last attempt for that message.
type EndpointMessageOut struct {
	Channels    []string       `json:"channels,omitempty"` // List of free-form identifiers that endpoints can filter by
	EventId     *string        `json:"eventId,omitempty"`  // Optional unique identifier for the message
	EventType   string         `json:"eventType"`          // The event type's name
	Id          string         `json:"id"`                 // The Message's ID.
	NextAttempt *time.Time     `json:"nextAttempt,omitempty"`
	Payload     map[string]any `json:"payload"`
	Status      MessageStatus  `json:"status"`
	Tags        []string       `json:"tags,omitempty"`
	Timestamp   time.Time      `json:"timestamp"`
}
