// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type ApplicationPatch struct {
	Metadata  *map[string]string     `json:"metadata,omitempty"`
	Name      *string                `json:"name,omitempty"`
	RateLimit utils.Nullable[uint16] `json:"rateLimit"`
	Uid       utils.Nullable[string] `json:"uid"` // The Application's UID.
}

func (o ApplicationPatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Metadata != nil {
		toSerialize["metadata"] = o.Metadata
	}
	if o.Name != nil {
		toSerialize["name"] = o.Name
	}
	if o.RateLimit.IsSet() {
		toSerialize["rateLimit"] = o.RateLimit
	}
	if o.Uid.IsSet() {
		toSerialize["uid"] = o.Uid
	}
	return json.Marshal(toSerialize)
}
