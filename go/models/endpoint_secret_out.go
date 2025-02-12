// Package svix this file is @generated DO NOT EDIT
package models

type EndpointSecretOut struct {
	// The endpoint's verification secret.
	//
	// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
	// It is recommended to not set this and let the server generate the secret.
	Key string `json:"key"`
}
