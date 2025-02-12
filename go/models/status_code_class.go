// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

// The different classes of HTTP status codes:
// - CodeNone = 0
// - Code1xx = 100
// - Code2xx = 200
// - Code3xx = 300
// - Code4xx = 400
// - Code5xx = 500
type StatusCodeClass int64

const (
	STATUSCODECLASS_CODE_NONE StatusCodeClass = 0
	STATUSCODECLASS_CODE1XX   StatusCodeClass = 100
	STATUSCODECLASS_CODE2XX   StatusCodeClass = 200
	STATUSCODECLASS_CODE3XX   StatusCodeClass = 300
	STATUSCODECLASS_CODE4XX   StatusCodeClass = 400
	STATUSCODECLASS_CODE5XX   StatusCodeClass = 500
)

var allowedStatusCodeClass = []StatusCodeClass{
	0,
	100,
	200,
	300,
	400,
	500,
}

func (v *StatusCodeClass) UnmarshalJSON(src []byte) error {
	var value int64
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := StatusCodeClass(value)
	if slices.Contains(allowedStatusCodeClass, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid StatusCodeClass", value)

}
