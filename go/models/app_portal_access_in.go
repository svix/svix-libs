// Package svix this file is @generated DO NOT EDIT
package models

type AppPortalAccessIn struct {
	// Optionally creates a new application while generating the access link.
	//
	// If the application id or uid that is used in the path already exists, this argument is ignored.
	Application *ApplicationIn `json:"application,omitempty"`
	// How long the token will be valid for, in seconds.
	//
	// Valid values are between 1 hour and 7 days. The default is 7 days.
	Expiry       *uint64  `json:"expiry,omitempty"`
	FeatureFlags []string `json:"featureFlags,omitempty"` // The set of feature flags the created token will have access to.
	ReadOnly     *bool    `json:"readOnly,omitempty"`     // Whether the app portal should be in read-only mode.
}
