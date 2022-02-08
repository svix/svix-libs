/*
 * Svix API
 *
 * Welcome to the Svix API documentation!  Useful links: [Homepage](https://www.svix.com) | [Support email](mailto:support+docs@svix.com) | [Blog](https://www.svix.com/blog/) | [Slack Community](https://www.svix.com/slack/)  # Introduction  This is the reference documentation and schemas for the [Svix webhook service](https://www.svix.com) API. For tutorials and other documentation please refer to [the documentation](https://docs.svix.com).  ## Main concepts  In Svix you have four important entities you will be interacting with:  - `messages`: these are the webhooks being sent. They can have contents and a few other properties. - `application`: this is where `messages` are sent to. Usually you want to create one application for each user on your platform. - `endpoint`: endpoints are the URLs messages will be sent to. Each application can have multiple `endpoints` and each message sent to that application will be sent to all of them (unless they are not subscribed to the sent event type). - `event-type`: event types are identifiers denoting the type of the message being sent. Event types are primarily used to decide which events are sent to which endpoint.   ## Authentication  Get your authentication token (`AUTH_TOKEN`) from the [Svix dashboard](https://dashboard.svix.com) and use it as part of the `Authorization` header as such: `Authorization: Bearer ${AUTH_TOKEN}`.  <SecurityDefinitions />   ## Code samples  The code samples assume you already have the respective libraries installed and you know how to use them. For the latest information on how to do that, please refer to [the documentation](https://docs.svix.com/).   ## Idempotency  Svix supports [idempotency](https://en.wikipedia.org/wiki/Idempotence) for safely retrying requests without accidentally performing the same operation twice. This is useful when an API call is disrupted in transit and you do not receive a response.  To perform an idempotent request, pass the idempotency key in the `Idempotency-Key` header to the request. The idempotency key should be a unique value generated by the client. You can create the key in however way you like, though we suggest using UUID v4, or any other string with enough entropy to avoid collisions.  Svix's idempotency works by saving the resulting status code and body of the first request made for any given idempotency key for any successful request. Subsequent requests with the same key return the same result.  Please note that idempotency is only supported for `POST` requests.   ## Cross-Origin Resource Sharing  This API features Cross-Origin Resource Sharing (CORS) implemented in compliance with [W3C spec](https://www.w3.org/TR/cors/). And that allows cross-domain communication from the browser. All responses have a wildcard same-origin which makes them completely public and accessible to everyone, including any code on any site. 
 *
 * API version: 1.4
 */

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
)

// SettingsOut struct for SettingsOut
type SettingsOut struct {
	CustomBaseFontSize *int32 `json:"customBaseFontSize,omitempty"`
	CustomColor *string `json:"customColor,omitempty"`
	CustomFontFamily *string `json:"customFontFamily,omitempty"`
	CustomLogoUrl *string `json:"customLogoUrl,omitempty"`
	CustomThemeOverride *CustomThemeOverride `json:"customThemeOverride,omitempty"`
	DisableEndpointOnFailure *bool `json:"disableEndpointOnFailure,omitempty"`
	EnableChannels *bool `json:"enableChannels,omitempty"`
	EnableIntegrationManagement *bool `json:"enableIntegrationManagement,omitempty"`
	EnforceHttps *bool `json:"enforceHttps,omitempty"`
}

// NewSettingsOut instantiates a new SettingsOut object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewSettingsOut() *SettingsOut {
	this := SettingsOut{}
	var disableEndpointOnFailure bool = true
	this.DisableEndpointOnFailure = &disableEndpointOnFailure
	var enableChannels bool = false
	this.EnableChannels = &enableChannels
	var enableIntegrationManagement bool = false
	this.EnableIntegrationManagement = &enableIntegrationManagement
	var enforceHttps bool = true
	this.EnforceHttps = &enforceHttps
	return &this
}

// NewSettingsOutWithDefaults instantiates a new SettingsOut object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewSettingsOutWithDefaults() *SettingsOut {
	this := SettingsOut{}
	var disableEndpointOnFailure bool = true
	this.DisableEndpointOnFailure = &disableEndpointOnFailure
	var enableChannels bool = false
	this.EnableChannels = &enableChannels
	var enableIntegrationManagement bool = false
	this.EnableIntegrationManagement = &enableIntegrationManagement
	var enforceHttps bool = true
	this.EnforceHttps = &enforceHttps
	return &this
}

// GetCustomBaseFontSize returns the CustomBaseFontSize field value if set, zero value otherwise.
func (o *SettingsOut) GetCustomBaseFontSize() int32 {
	if o == nil || o.CustomBaseFontSize == nil {
		var ret int32
		return ret
	}
	return *o.CustomBaseFontSize
}

// GetCustomBaseFontSizeOk returns a tuple with the CustomBaseFontSize field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetCustomBaseFontSizeOk() (*int32, bool) {
	if o == nil || o.CustomBaseFontSize == nil {
		return nil, false
	}
	return o.CustomBaseFontSize, true
}

// HasCustomBaseFontSize returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomBaseFontSize() bool {
	if o != nil && o.CustomBaseFontSize != nil {
		return true
	}

	return false
}

// SetCustomBaseFontSize gets a reference to the given int32 and assigns it to the CustomBaseFontSize field.
func (o *SettingsOut) SetCustomBaseFontSize(v int32) {
	o.CustomBaseFontSize = &v
}

// GetCustomColor returns the CustomColor field value if set, zero value otherwise.
func (o *SettingsOut) GetCustomColor() string {
	if o == nil || o.CustomColor == nil {
		var ret string
		return ret
	}
	return *o.CustomColor
}

// GetCustomColorOk returns a tuple with the CustomColor field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetCustomColorOk() (*string, bool) {
	if o == nil || o.CustomColor == nil {
		return nil, false
	}
	return o.CustomColor, true
}

// HasCustomColor returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomColor() bool {
	if o != nil && o.CustomColor != nil {
		return true
	}

	return false
}

// SetCustomColor gets a reference to the given string and assigns it to the CustomColor field.
func (o *SettingsOut) SetCustomColor(v string) {
	o.CustomColor = &v
}

// GetCustomFontFamily returns the CustomFontFamily field value if set, zero value otherwise.
func (o *SettingsOut) GetCustomFontFamily() string {
	if o == nil || o.CustomFontFamily == nil {
		var ret string
		return ret
	}
	return *o.CustomFontFamily
}

// GetCustomFontFamilyOk returns a tuple with the CustomFontFamily field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetCustomFontFamilyOk() (*string, bool) {
	if o == nil || o.CustomFontFamily == nil {
		return nil, false
	}
	return o.CustomFontFamily, true
}

// HasCustomFontFamily returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomFontFamily() bool {
	if o != nil && o.CustomFontFamily != nil {
		return true
	}

	return false
}

// SetCustomFontFamily gets a reference to the given string and assigns it to the CustomFontFamily field.
func (o *SettingsOut) SetCustomFontFamily(v string) {
	o.CustomFontFamily = &v
}

// GetCustomLogoUrl returns the CustomLogoUrl field value if set, zero value otherwise.
func (o *SettingsOut) GetCustomLogoUrl() string {
	if o == nil || o.CustomLogoUrl == nil {
		var ret string
		return ret
	}
	return *o.CustomLogoUrl
}

// GetCustomLogoUrlOk returns a tuple with the CustomLogoUrl field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetCustomLogoUrlOk() (*string, bool) {
	if o == nil || o.CustomLogoUrl == nil {
		return nil, false
	}
	return o.CustomLogoUrl, true
}

// HasCustomLogoUrl returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomLogoUrl() bool {
	if o != nil && o.CustomLogoUrl != nil {
		return true
	}

	return false
}

// SetCustomLogoUrl gets a reference to the given string and assigns it to the CustomLogoUrl field.
func (o *SettingsOut) SetCustomLogoUrl(v string) {
	o.CustomLogoUrl = &v
}

// GetCustomThemeOverride returns the CustomThemeOverride field value if set, zero value otherwise.
func (o *SettingsOut) GetCustomThemeOverride() CustomThemeOverride {
	if o == nil || o.CustomThemeOverride == nil {
		var ret CustomThemeOverride
		return ret
	}
	return *o.CustomThemeOverride
}

// GetCustomThemeOverrideOk returns a tuple with the CustomThemeOverride field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetCustomThemeOverrideOk() (*CustomThemeOverride, bool) {
	if o == nil || o.CustomThemeOverride == nil {
		return nil, false
	}
	return o.CustomThemeOverride, true
}

// HasCustomThemeOverride returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomThemeOverride() bool {
	if o != nil && o.CustomThemeOverride != nil {
		return true
	}

	return false
}

// SetCustomThemeOverride gets a reference to the given CustomThemeOverride and assigns it to the CustomThemeOverride field.
func (o *SettingsOut) SetCustomThemeOverride(v CustomThemeOverride) {
	o.CustomThemeOverride = &v
}

// GetDisableEndpointOnFailure returns the DisableEndpointOnFailure field value if set, zero value otherwise.
func (o *SettingsOut) GetDisableEndpointOnFailure() bool {
	if o == nil || o.DisableEndpointOnFailure == nil {
		var ret bool
		return ret
	}
	return *o.DisableEndpointOnFailure
}

// GetDisableEndpointOnFailureOk returns a tuple with the DisableEndpointOnFailure field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetDisableEndpointOnFailureOk() (*bool, bool) {
	if o == nil || o.DisableEndpointOnFailure == nil {
		return nil, false
	}
	return o.DisableEndpointOnFailure, true
}

// HasDisableEndpointOnFailure returns a boolean if a field has been set.
func (o *SettingsOut) HasDisableEndpointOnFailure() bool {
	if o != nil && o.DisableEndpointOnFailure != nil {
		return true
	}

	return false
}

// SetDisableEndpointOnFailure gets a reference to the given bool and assigns it to the DisableEndpointOnFailure field.
func (o *SettingsOut) SetDisableEndpointOnFailure(v bool) {
	o.DisableEndpointOnFailure = &v
}

// GetEnableChannels returns the EnableChannels field value if set, zero value otherwise.
func (o *SettingsOut) GetEnableChannels() bool {
	if o == nil || o.EnableChannels == nil {
		var ret bool
		return ret
	}
	return *o.EnableChannels
}

// GetEnableChannelsOk returns a tuple with the EnableChannels field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetEnableChannelsOk() (*bool, bool) {
	if o == nil || o.EnableChannels == nil {
		return nil, false
	}
	return o.EnableChannels, true
}

// HasEnableChannels returns a boolean if a field has been set.
func (o *SettingsOut) HasEnableChannels() bool {
	if o != nil && o.EnableChannels != nil {
		return true
	}

	return false
}

// SetEnableChannels gets a reference to the given bool and assigns it to the EnableChannels field.
func (o *SettingsOut) SetEnableChannels(v bool) {
	o.EnableChannels = &v
}

// GetEnableIntegrationManagement returns the EnableIntegrationManagement field value if set, zero value otherwise.
func (o *SettingsOut) GetEnableIntegrationManagement() bool {
	if o == nil || o.EnableIntegrationManagement == nil {
		var ret bool
		return ret
	}
	return *o.EnableIntegrationManagement
}

// GetEnableIntegrationManagementOk returns a tuple with the EnableIntegrationManagement field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetEnableIntegrationManagementOk() (*bool, bool) {
	if o == nil || o.EnableIntegrationManagement == nil {
		return nil, false
	}
	return o.EnableIntegrationManagement, true
}

// HasEnableIntegrationManagement returns a boolean if a field has been set.
func (o *SettingsOut) HasEnableIntegrationManagement() bool {
	if o != nil && o.EnableIntegrationManagement != nil {
		return true
	}

	return false
}

// SetEnableIntegrationManagement gets a reference to the given bool and assigns it to the EnableIntegrationManagement field.
func (o *SettingsOut) SetEnableIntegrationManagement(v bool) {
	o.EnableIntegrationManagement = &v
}

// GetEnforceHttps returns the EnforceHttps field value if set, zero value otherwise.
func (o *SettingsOut) GetEnforceHttps() bool {
	if o == nil || o.EnforceHttps == nil {
		var ret bool
		return ret
	}
	return *o.EnforceHttps
}

// GetEnforceHttpsOk returns a tuple with the EnforceHttps field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetEnforceHttpsOk() (*bool, bool) {
	if o == nil || o.EnforceHttps == nil {
		return nil, false
	}
	return o.EnforceHttps, true
}

// HasEnforceHttps returns a boolean if a field has been set.
func (o *SettingsOut) HasEnforceHttps() bool {
	if o != nil && o.EnforceHttps != nil {
		return true
	}

	return false
}

// SetEnforceHttps gets a reference to the given bool and assigns it to the EnforceHttps field.
func (o *SettingsOut) SetEnforceHttps(v bool) {
	o.EnforceHttps = &v
}

func (o SettingsOut) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.CustomBaseFontSize != nil {
		toSerialize["customBaseFontSize"] = o.CustomBaseFontSize
	}
	if o.CustomColor != nil {
		toSerialize["customColor"] = o.CustomColor
	}
	if o.CustomFontFamily != nil {
		toSerialize["customFontFamily"] = o.CustomFontFamily
	}
	if o.CustomLogoUrl != nil {
		toSerialize["customLogoUrl"] = o.CustomLogoUrl
	}
	if o.CustomThemeOverride != nil {
		toSerialize["customThemeOverride"] = o.CustomThemeOverride
	}
	if o.DisableEndpointOnFailure != nil {
		toSerialize["disableEndpointOnFailure"] = o.DisableEndpointOnFailure
	}
	if o.EnableChannels != nil {
		toSerialize["enableChannels"] = o.EnableChannels
	}
	if o.EnableIntegrationManagement != nil {
		toSerialize["enableIntegrationManagement"] = o.EnableIntegrationManagement
	}
	if o.EnforceHttps != nil {
		toSerialize["enforceHttps"] = o.EnforceHttps
	}
	return json.Marshal(toSerialize)
}

type NullableSettingsOut struct {
	value *SettingsOut
	isSet bool
}

func (v NullableSettingsOut) Get() *SettingsOut {
	return v.value
}

func (v *NullableSettingsOut) Set(val *SettingsOut) {
	v.value = val
	v.isSet = true
}

func (v NullableSettingsOut) IsSet() bool {
	return v.isSet
}

func (v *NullableSettingsOut) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableSettingsOut(val *SettingsOut) *NullableSettingsOut {
	return &NullableSettingsOut{value: val, isSet: true}
}

func (v NullableSettingsOut) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableSettingsOut) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


