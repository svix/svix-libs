// Package svix this file is @generated DO NOT EDIT
package models

import "time"

type IntegrationOut struct {
	CreatedAt time.Time `json:"createdAt"`
	Id        string    `json:"id"` // The integ's ID
	Name      string    `json:"name"`
	UpdatedAt time.Time `json:"updatedAt"`
}
