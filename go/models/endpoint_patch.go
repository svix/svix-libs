// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type EndpointPatch struct {
	Channels    utils.Nullable[[]string] `json:"channels"`
	Description *string                  `json:"description,omitempty"`
	Disabled    *bool                    `json:"disabled,omitempty"`
	FilterTypes utils.Nullable[[]string] `json:"filterTypes"`
	Metadata    *map[string]string       `json:"metadata,omitempty"`
	RateLimit   utils.Nullable[uint16]   `json:"rateLimit"`
	// The endpoint's verification secret.
	//
	// Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
	// It is recommended to not set this and let the server generate the secret.
	Secret  utils.Nullable[string] `json:"secret"`
	Uid     utils.Nullable[string] `json:"uid"` // The Endpoint's UID.
	Url     *string                `json:"url,omitempty"`
	Version *uint16                `json:"version,omitempty"`
}

func (o EndpointPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Channels.IsSet() {
		toSerialize["channels"] = o.Channels
	}
	if o.Description != nil {
		toSerialize["description"] = o.Description
	}
	if o.Disabled != nil {
		toSerialize["disabled"] = o.Disabled
	}
	if o.FilterTypes.IsSet() {
		toSerialize["filterTypes"] = o.FilterTypes
	}
	if o.Metadata != nil {
		toSerialize["metadata"] = o.Metadata
	}
	if o.RateLimit.IsSet() {
		toSerialize["rateLimit"] = o.RateLimit
	}
	if o.Secret.IsSet() {
		toSerialize["secret"] = o.Secret
	}
	if o.Uid.IsSet() {
		toSerialize["uid"] = o.Uid
	}
	if o.Url != nil {
		toSerialize["url"] = o.Url
	}
	if o.Version != nil {
		toSerialize["version"] = o.Version
	}
	return json.Marshal(toSerialize)
}
