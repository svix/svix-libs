// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type BackgroundTaskStatus string

const (
	BACKGROUNDTASKSTATUS_RUNNING  BackgroundTaskStatus = "running"
	BACKGROUNDTASKSTATUS_FINISHED BackgroundTaskStatus = "finished"
	BACKGROUNDTASKSTATUS_FAILED   BackgroundTaskStatus = "failed"
)

var allowedBackgroundTaskStatus = []BackgroundTaskStatus{
	"running",
	"finished",
	"failed",
}

func (v *BackgroundTaskStatus) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := BackgroundTaskStatus(value)
	if slices.Contains(allowedBackgroundTaskStatus, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid BackgroundTaskStatus", value)

}
