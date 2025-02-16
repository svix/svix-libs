// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type ConnectorKind string

const (
	CONNECTORKIND_CUSTOM      ConnectorKind = "Custom"
	CONNECTORKIND_CUSTOMER_IO ConnectorKind = "CustomerIO"
	CONNECTORKIND_DISCORD     ConnectorKind = "Discord"
	CONNECTORKIND_HUBSPOT     ConnectorKind = "Hubspot"
	CONNECTORKIND_INNGEST     ConnectorKind = "Inngest"
	CONNECTORKIND_SALESFORCE  ConnectorKind = "Salesforce"
	CONNECTORKIND_SEGMENT     ConnectorKind = "Segment"
	CONNECTORKIND_SLACK       ConnectorKind = "Slack"
	CONNECTORKIND_TEAMS       ConnectorKind = "Teams"
	CONNECTORKIND_TRIGGER_DEV ConnectorKind = "TriggerDev"
	CONNECTORKIND_WINDMILL    ConnectorKind = "Windmill"
	CONNECTORKIND_ZAPIER      ConnectorKind = "Zapier"
)

var allowedConnectorKind = []ConnectorKind{
	"Custom",
	"CustomerIO",
	"Discord",
	"Hubspot",
	"Inngest",
	"Salesforce",
	"Segment",
	"Slack",
	"Teams",
	"TriggerDev",
	"Windmill",
	"Zapier",
}

func (v *ConnectorKind) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := ConnectorKind(value)
	if slices.Contains(allowedConnectorKind, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid ConnectorKind", value)

}
