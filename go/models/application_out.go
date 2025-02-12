// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ApplicationOut struct {
	CreatedAt time.Time         `json:"createdAt"`
	Id        string            `json:"id"` // The app's ID
	Metadata  map[string]string `json:"metadata"`
	Name      string            `json:"name"`
	RateLimit *uint16           `json:"rateLimit,omitempty"`
	Uid       *string           `json:"uid,omitempty"` // The app's UID
	UpdatedAt time.Time         `json:"updatedAt"`
}
