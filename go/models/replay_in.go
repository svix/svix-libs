// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type ReplayIn struct {
	Since time.Time  `json:"since"`
	Until *time.Time `json:"until,omitempty"`
}
