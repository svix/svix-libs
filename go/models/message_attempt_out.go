// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type MessageAttemptOut struct {
	EndpointId         string                    `json:"endpointId"` // The Endpoint's ID.
	Id                 string                    `json:"id"`         // The MessageAttempt's ID.
	Msg                *MessageOut               `json:"msg,omitempty"`
	MsgId              string                    `json:"msgId"` // The Message's ID.
	Response           string                    `json:"response"`
	ResponseDurationMs int64                     `json:"responseDurationMs"` // Response duration in milliseconds.
	ResponseStatusCode int16                     `json:"responseStatusCode"`
	Status             MessageStatus             `json:"status"`
	Timestamp          time.Time                 `json:"timestamp"`
	TriggerType        MessageAttemptTriggerType `json:"triggerType"`
	Url                string                    `json:"url"`
}
