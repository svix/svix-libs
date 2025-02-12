// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

// Defines the ordering in a listing of results.
type Ordering string

const (
	ORDERING_ASCENDING  Ordering = "ascending"
	ORDERING_DESCENDING Ordering = "descending"
)

var allowedOrdering = []Ordering{
	"ascending",
	"descending",
}

func (v *Ordering) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := Ordering(value)
	if slices.Contains(allowedOrdering, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid Ordering", value)

}
