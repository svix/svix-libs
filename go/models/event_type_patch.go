// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"

	"github.com/svix/svix-webhooks/go/utils"
)

type EventTypePatch struct {
	Archived    *bool                          `json:"archived,omitempty"`
	Deprecated  *bool                          `json:"deprecated,omitempty"`
	Description *string                        `json:"description,omitempty"`
	FeatureFlag utils.Nullable[string]         `json:"featureFlag"`
	GroupName   utils.Nullable[string]         `json:"groupName"` // The event type group's name
	Schemas     utils.Nullable[map[string]any] `json:"schemas"`
}

func (o EventTypePatch) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.Archived != nil {
		toSerialize["archived"] = o.Archived
	}
	if o.Deprecated != nil {
		toSerialize["deprecated"] = o.Deprecated
	}
	if o.Description != nil {
		toSerialize["description"] = o.Description
	}
	if o.FeatureFlag.IsSet() {
		toSerialize["featureFlag"] = o.FeatureFlag
	}
	if o.GroupName.IsSet() {
		toSerialize["groupName"] = o.GroupName
	}
	if o.Schemas.IsSet() {
		toSerialize["schemas"] = o.Schemas
	}
	return json.Marshal(toSerialize)
}
