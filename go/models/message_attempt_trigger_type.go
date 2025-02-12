// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

// The reason an attempt was made:
// - Scheduled = 0
// - Manual = 1
type MessageAttemptTriggerType int64

const (
	MESSAGEATTEMPTTRIGGERTYPE_SCHEDULED MessageAttemptTriggerType = 0
	MESSAGEATTEMPTTRIGGERTYPE_MANUAL    MessageAttemptTriggerType = 1
)

var allowedMessageAttemptTriggerType = []MessageAttemptTriggerType{
	0,
	1,
}

func (v *MessageAttemptTriggerType) UnmarshalJSON(src []byte) error {
	var value int64
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := MessageAttemptTriggerType(value)
	if slices.Contains(allowedMessageAttemptTriggerType, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid MessageAttemptTriggerType", value)

}
