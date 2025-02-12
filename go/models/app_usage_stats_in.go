// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type AppUsageStatsIn struct {
	// Specific app IDs or UIDs to aggregate stats for.
	//
	// Note that if none of the given IDs or UIDs are resolved, a 422 response will be given.
	AppIds []string  `json:"appIds,omitempty"`
	Since  time.Time `json:"since"`
	Until  time.Time `json:"until"`
}
