// Package svix this file is @generated DO NOT EDIT
package models

import (
	"encoding/json"
	"fmt"
	"slices"
)

type TransformationTemplateKind string

const (
	TRANSFORMATIONTEMPLATEKIND_CUSTOM      TransformationTemplateKind = "Custom"
	TRANSFORMATIONTEMPLATEKIND_CUSTOMER_IO TransformationTemplateKind = "CustomerIO"
	TRANSFORMATIONTEMPLATEKIND_DISCORD     TransformationTemplateKind = "Discord"
	TRANSFORMATIONTEMPLATEKIND_HUBSPOT     TransformationTemplateKind = "Hubspot"
	TRANSFORMATIONTEMPLATEKIND_INNGEST     TransformationTemplateKind = "Inngest"
	TRANSFORMATIONTEMPLATEKIND_SALESFORCE  TransformationTemplateKind = "Salesforce"
	TRANSFORMATIONTEMPLATEKIND_SEGMENT     TransformationTemplateKind = "Segment"
	TRANSFORMATIONTEMPLATEKIND_SLACK       TransformationTemplateKind = "Slack"
	TRANSFORMATIONTEMPLATEKIND_TEAMS       TransformationTemplateKind = "Teams"
	TRANSFORMATIONTEMPLATEKIND_TRIGGER_DEV TransformationTemplateKind = "TriggerDev"
	TRANSFORMATIONTEMPLATEKIND_WINDMILL    TransformationTemplateKind = "Windmill"
	TRANSFORMATIONTEMPLATEKIND_ZAPIER      TransformationTemplateKind = "Zapier"
)

var allowedTransformationTemplateKind = []TransformationTemplateKind{
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

func (v *TransformationTemplateKind) UnmarshalJSON(src []byte) error {
	var value string
	err := json.Unmarshal(src, &value)
	if err != nil {
		return err
	}
	enumVal := TransformationTemplateKind(value)
	if slices.Contains(allowedTransformationTemplateKind, enumVal) {
		*v = enumVal
		return nil
	}
	return fmt.Errorf("`%+v` is not a valid TransformationTemplateKind", value)

}
