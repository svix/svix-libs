// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

// The sending status of the message:
// - Success = 0
// - Pending = 1
// - Fail = 2
// - Sending = 3
type MessageStatus int64

const (
	MESSAGESTATUS_SUCCESS MessageStatus = 0
	MESSAGESTATUS_PENDING MessageStatus = 1
	MESSAGESTATUS_FAIL    MessageStatus = 2
	MESSAGESTATUS_SENDING MessageStatus = 3
)

var allowedMessageStatus = []MessageStatus{
	0,
	1,
	2,
	3,
}

func (v *MessageStatus) UnmarshalJSON(src []byte) error {
	var value int64
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := MessageStatus(value)
	if slices.Contains(allowedMessageStatus, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid MessageStatus", value)

}
