// Package svix this file is @generated DO NOT EDIT
package models

type MessageIn struct {
	// Optionally creates a new application alongside the message.
	//
	// If the application id or uid that is used in the path already exists, this argument is ignored.
	Application *ApplicationIn `json:"application,omitempty"`
	Channels    []string       `json:"channels,omitempty"` // List of free-form identifiers that endpoints can filter by
	EventId     *string        `json:"eventId,omitempty"`  // Optional unique identifier for the message
	EventType   string         `json:"eventType"`          // The event type's name
	// JSON payload to send as the request body of the webhook.
	//
	// We also support sending non-JSON payloads. Please contact us for more information.
	Payload                map[string]any  `json:"payload"`
	PayloadRetentionHours  *int64          `json:"payloadRetentionHours,omitempty"`  // Optional number of hours to retain the message payload. Note that this is mutually exclusive with `payloadRetentionPeriod`.
	PayloadRetentionPeriod *int64          `json:"payloadRetentionPeriod,omitempty"` // Optional number of days to retain the message payload. Defaults to 90. Note that this is mutually exclusive with `payloadRetentionHours`.
	Tags                   []string        `json:"tags,omitempty"`                   // List of free-form tags that can be filtered by when listing messages
	TransformationsParams  *map[string]any `json:"transformationsParams,omitempty"`  // Extra parameters to pass to Transformations (for future use)
}
