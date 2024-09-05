/*
 * Svix API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * API version: 1.1.1
 */

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
)

// SettingsOut struct for SettingsOut
type SettingsOut struct {
	ColorPaletteDark *CustomColorPalette `json:"colorPaletteDark,omitempty"`
	ColorPaletteLight *CustomColorPalette `json:"colorPaletteLight,omitempty"`
	CustomBaseFontSize NullableInt32 `json:"customBaseFontSize,omitempty"`
	CustomColor NullableString `json:"customColor,omitempty"`
	CustomFontFamily NullableString `json:"customFontFamily,omitempty"`
	CustomFontFamilyUrl NullableString `json:"customFontFamilyUrl,omitempty"`
	CustomLogoUrl NullableString `json:"customLogoUrl,omitempty"`
	CustomStringsOverride *CustomStringsOverride `json:"customStringsOverride,omitempty"`
	CustomThemeOverride *CustomThemeOverride `json:"customThemeOverride,omitempty"`
	DisableEndpointOnFailure *bool `json:"disableEndpointOnFailure,omitempty"`
	DisplayName NullableString `json:"displayName,omitempty"`
	EnableChannels *bool `json:"enableChannels,omitempty"`
	EnableIntegrationManagement *bool `json:"enableIntegrationManagement,omitempty"`
	EnableTransformations *bool `json:"enableTransformations,omitempty"`
	EnforceHttps *bool `json:"enforceHttps,omitempty"`
	EventCatalogPublished *bool `json:"eventCatalogPublished,omitempty"`
	ReadOnly *bool `json:"readOnly,omitempty"`
	ShowUseSvixPlay *bool `json:"showUseSvixPlay,omitempty"`
	WipeSuccessfulPayload *bool `json:"wipeSuccessfulPayload,omitempty"`
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
	var enableTransformations bool = false
	this.EnableTransformations = &enableTransformations
	var enforceHttps bool = true
	this.EnforceHttps = &enforceHttps
	var eventCatalogPublished bool = false
	this.EventCatalogPublished = &eventCatalogPublished
	var readOnly bool = false
	this.ReadOnly = &readOnly
	var showUseSvixPlay bool = true
	this.ShowUseSvixPlay = &showUseSvixPlay
	var wipeSuccessfulPayload bool = false
	this.WipeSuccessfulPayload = &wipeSuccessfulPayload
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
	var enableTransformations bool = false
	this.EnableTransformations = &enableTransformations
	var enforceHttps bool = true
	this.EnforceHttps = &enforceHttps
	var eventCatalogPublished bool = false
	this.EventCatalogPublished = &eventCatalogPublished
	var readOnly bool = false
	this.ReadOnly = &readOnly
	var showUseSvixPlay bool = true
	this.ShowUseSvixPlay = &showUseSvixPlay
	var wipeSuccessfulPayload bool = false
	this.WipeSuccessfulPayload = &wipeSuccessfulPayload
	return &this
}

// GetColorPaletteDark returns the ColorPaletteDark field value if set, zero value otherwise.
func (o *SettingsOut) GetColorPaletteDark() CustomColorPalette {
	if o == nil || o.ColorPaletteDark == nil {
		var ret CustomColorPalette
		return ret
	}
	return *o.ColorPaletteDark
}

// GetColorPaletteDarkOk returns a tuple with the ColorPaletteDark field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetColorPaletteDarkOk() (*CustomColorPalette, bool) {
	if o == nil || o.ColorPaletteDark == nil {
		return nil, false
	}
	return o.ColorPaletteDark, true
}

// HasColorPaletteDark returns a boolean if a field has been set.
func (o *SettingsOut) HasColorPaletteDark() bool {
	if o != nil && o.ColorPaletteDark != nil {
		return true
	}

	return false
}

// SetColorPaletteDark gets a reference to the given CustomColorPalette and assigns it to the ColorPaletteDark field.
func (o *SettingsOut) SetColorPaletteDark(v CustomColorPalette) {
	o.ColorPaletteDark = &v
}

// GetColorPaletteLight returns the ColorPaletteLight field value if set, zero value otherwise.
func (o *SettingsOut) GetColorPaletteLight() CustomColorPalette {
	if o == nil || o.ColorPaletteLight == nil {
		var ret CustomColorPalette
		return ret
	}
	return *o.ColorPaletteLight
}

// GetColorPaletteLightOk returns a tuple with the ColorPaletteLight field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetColorPaletteLightOk() (*CustomColorPalette, bool) {
	if o == nil || o.ColorPaletteLight == nil {
		return nil, false
	}
	return o.ColorPaletteLight, true
}

// HasColorPaletteLight returns a boolean if a field has been set.
func (o *SettingsOut) HasColorPaletteLight() bool {
	if o != nil && o.ColorPaletteLight != nil {
		return true
	}

	return false
}

// SetColorPaletteLight gets a reference to the given CustomColorPalette and assigns it to the ColorPaletteLight field.
func (o *SettingsOut) SetColorPaletteLight(v CustomColorPalette) {
	o.ColorPaletteLight = &v
}

// GetCustomBaseFontSize returns the CustomBaseFontSize field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *SettingsOut) GetCustomBaseFontSize() int32 {
	if o == nil || o.CustomBaseFontSize.Get() == nil {
		var ret int32
		return ret
	}
	return *o.CustomBaseFontSize.Get()
}

// GetCustomBaseFontSizeOk returns a tuple with the CustomBaseFontSize field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *SettingsOut) GetCustomBaseFontSizeOk() (*int32, bool) {
	if o == nil  {
		return nil, false
	}
	return o.CustomBaseFontSize.Get(), o.CustomBaseFontSize.IsSet()
}

// HasCustomBaseFontSize returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomBaseFontSize() bool {
	if o != nil && o.CustomBaseFontSize.IsSet() {
		return true
	}

	return false
}

// SetCustomBaseFontSize gets a reference to the given NullableInt32 and assigns it to the CustomBaseFontSize field.
func (o *SettingsOut) SetCustomBaseFontSize(v int32) {
	o.CustomBaseFontSize.Set(&v)
}
// SetCustomBaseFontSizeNil sets the value for CustomBaseFontSize to be an explicit nil
func (o *SettingsOut) SetCustomBaseFontSizeNil() {
	o.CustomBaseFontSize.Set(nil)
}

// UnsetCustomBaseFontSize ensures that no value is present for CustomBaseFontSize, not even an explicit nil
func (o *SettingsOut) UnsetCustomBaseFontSize() {
	o.CustomBaseFontSize.Unset()
}

// GetCustomColor returns the CustomColor field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *SettingsOut) GetCustomColor() string {
	if o == nil || o.CustomColor.Get() == nil {
		var ret string
		return ret
	}
	return *o.CustomColor.Get()
}

// GetCustomColorOk returns a tuple with the CustomColor field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *SettingsOut) GetCustomColorOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.CustomColor.Get(), o.CustomColor.IsSet()
}

// HasCustomColor returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomColor() bool {
	if o != nil && o.CustomColor.IsSet() {
		return true
	}

	return false
}

// SetCustomColor gets a reference to the given NullableString and assigns it to the CustomColor field.
func (o *SettingsOut) SetCustomColor(v string) {
	o.CustomColor.Set(&v)
}
// SetCustomColorNil sets the value for CustomColor to be an explicit nil
func (o *SettingsOut) SetCustomColorNil() {
	o.CustomColor.Set(nil)
}

// UnsetCustomColor ensures that no value is present for CustomColor, not even an explicit nil
func (o *SettingsOut) UnsetCustomColor() {
	o.CustomColor.Unset()
}

// GetCustomFontFamily returns the CustomFontFamily field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *SettingsOut) GetCustomFontFamily() string {
	if o == nil || o.CustomFontFamily.Get() == nil {
		var ret string
		return ret
	}
	return *o.CustomFontFamily.Get()
}

// GetCustomFontFamilyOk returns a tuple with the CustomFontFamily field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *SettingsOut) GetCustomFontFamilyOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.CustomFontFamily.Get(), o.CustomFontFamily.IsSet()
}

// HasCustomFontFamily returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomFontFamily() bool {
	if o != nil && o.CustomFontFamily.IsSet() {
		return true
	}

	return false
}

// SetCustomFontFamily gets a reference to the given NullableString and assigns it to the CustomFontFamily field.
func (o *SettingsOut) SetCustomFontFamily(v string) {
	o.CustomFontFamily.Set(&v)
}
// SetCustomFontFamilyNil sets the value for CustomFontFamily to be an explicit nil
func (o *SettingsOut) SetCustomFontFamilyNil() {
	o.CustomFontFamily.Set(nil)
}

// UnsetCustomFontFamily ensures that no value is present for CustomFontFamily, not even an explicit nil
func (o *SettingsOut) UnsetCustomFontFamily() {
	o.CustomFontFamily.Unset()
}

// GetCustomFontFamilyUrl returns the CustomFontFamilyUrl field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *SettingsOut) GetCustomFontFamilyUrl() string {
	if o == nil || o.CustomFontFamilyUrl.Get() == nil {
		var ret string
		return ret
	}
	return *o.CustomFontFamilyUrl.Get()
}

// GetCustomFontFamilyUrlOk returns a tuple with the CustomFontFamilyUrl field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *SettingsOut) GetCustomFontFamilyUrlOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.CustomFontFamilyUrl.Get(), o.CustomFontFamilyUrl.IsSet()
}

// HasCustomFontFamilyUrl returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomFontFamilyUrl() bool {
	if o != nil && o.CustomFontFamilyUrl.IsSet() {
		return true
	}

	return false
}

// SetCustomFontFamilyUrl gets a reference to the given NullableString and assigns it to the CustomFontFamilyUrl field.
func (o *SettingsOut) SetCustomFontFamilyUrl(v string) {
	o.CustomFontFamilyUrl.Set(&v)
}
// SetCustomFontFamilyUrlNil sets the value for CustomFontFamilyUrl to be an explicit nil
func (o *SettingsOut) SetCustomFontFamilyUrlNil() {
	o.CustomFontFamilyUrl.Set(nil)
}

// UnsetCustomFontFamilyUrl ensures that no value is present for CustomFontFamilyUrl, not even an explicit nil
func (o *SettingsOut) UnsetCustomFontFamilyUrl() {
	o.CustomFontFamilyUrl.Unset()
}

// GetCustomLogoUrl returns the CustomLogoUrl field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *SettingsOut) GetCustomLogoUrl() string {
	if o == nil || o.CustomLogoUrl.Get() == nil {
		var ret string
		return ret
	}
	return *o.CustomLogoUrl.Get()
}

// GetCustomLogoUrlOk returns a tuple with the CustomLogoUrl field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *SettingsOut) GetCustomLogoUrlOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.CustomLogoUrl.Get(), o.CustomLogoUrl.IsSet()
}

// HasCustomLogoUrl returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomLogoUrl() bool {
	if o != nil && o.CustomLogoUrl.IsSet() {
		return true
	}

	return false
}

// SetCustomLogoUrl gets a reference to the given NullableString and assigns it to the CustomLogoUrl field.
func (o *SettingsOut) SetCustomLogoUrl(v string) {
	o.CustomLogoUrl.Set(&v)
}
// SetCustomLogoUrlNil sets the value for CustomLogoUrl to be an explicit nil
func (o *SettingsOut) SetCustomLogoUrlNil() {
	o.CustomLogoUrl.Set(nil)
}

// UnsetCustomLogoUrl ensures that no value is present for CustomLogoUrl, not even an explicit nil
func (o *SettingsOut) UnsetCustomLogoUrl() {
	o.CustomLogoUrl.Unset()
}

// GetCustomStringsOverride returns the CustomStringsOverride field value if set, zero value otherwise.
func (o *SettingsOut) GetCustomStringsOverride() CustomStringsOverride {
	if o == nil || o.CustomStringsOverride == nil {
		var ret CustomStringsOverride
		return ret
	}
	return *o.CustomStringsOverride
}

// GetCustomStringsOverrideOk returns a tuple with the CustomStringsOverride field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetCustomStringsOverrideOk() (*CustomStringsOverride, bool) {
	if o == nil || o.CustomStringsOverride == nil {
		return nil, false
	}
	return o.CustomStringsOverride, true
}

// HasCustomStringsOverride returns a boolean if a field has been set.
func (o *SettingsOut) HasCustomStringsOverride() bool {
	if o != nil && o.CustomStringsOverride != nil {
		return true
	}

	return false
}

// SetCustomStringsOverride gets a reference to the given CustomStringsOverride and assigns it to the CustomStringsOverride field.
func (o *SettingsOut) SetCustomStringsOverride(v CustomStringsOverride) {
	o.CustomStringsOverride = &v
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

// GetDisplayName returns the DisplayName field value if set, zero value otherwise (both if not set or set to explicit null).
func (o *SettingsOut) GetDisplayName() string {
	if o == nil || o.DisplayName.Get() == nil {
		var ret string
		return ret
	}
	return *o.DisplayName.Get()
}

// GetDisplayNameOk returns a tuple with the DisplayName field value if set, nil otherwise
// and a boolean to check if the value has been set.
// NOTE: If the value is an explicit nil, `nil, true` will be returned
func (o *SettingsOut) GetDisplayNameOk() (*string, bool) {
	if o == nil  {
		return nil, false
	}
	return o.DisplayName.Get(), o.DisplayName.IsSet()
}

// HasDisplayName returns a boolean if a field has been set.
func (o *SettingsOut) HasDisplayName() bool {
	if o != nil && o.DisplayName.IsSet() {
		return true
	}

	return false
}

// SetDisplayName gets a reference to the given NullableString and assigns it to the DisplayName field.
func (o *SettingsOut) SetDisplayName(v string) {
	o.DisplayName.Set(&v)
}
// SetDisplayNameNil sets the value for DisplayName to be an explicit nil
func (o *SettingsOut) SetDisplayNameNil() {
	o.DisplayName.Set(nil)
}

// UnsetDisplayName ensures that no value is present for DisplayName, not even an explicit nil
func (o *SettingsOut) UnsetDisplayName() {
	o.DisplayName.Unset()
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

// GetEnableTransformations returns the EnableTransformations field value if set, zero value otherwise.
func (o *SettingsOut) GetEnableTransformations() bool {
	if o == nil || o.EnableTransformations == nil {
		var ret bool
		return ret
	}
	return *o.EnableTransformations
}

// GetEnableTransformationsOk returns a tuple with the EnableTransformations field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetEnableTransformationsOk() (*bool, bool) {
	if o == nil || o.EnableTransformations == nil {
		return nil, false
	}
	return o.EnableTransformations, true
}

// HasEnableTransformations returns a boolean if a field has been set.
func (o *SettingsOut) HasEnableTransformations() bool {
	if o != nil && o.EnableTransformations != nil {
		return true
	}

	return false
}

// SetEnableTransformations gets a reference to the given bool and assigns it to the EnableTransformations field.
func (o *SettingsOut) SetEnableTransformations(v bool) {
	o.EnableTransformations = &v
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

// GetEventCatalogPublished returns the EventCatalogPublished field value if set, zero value otherwise.
func (o *SettingsOut) GetEventCatalogPublished() bool {
	if o == nil || o.EventCatalogPublished == nil {
		var ret bool
		return ret
	}
	return *o.EventCatalogPublished
}

// GetEventCatalogPublishedOk returns a tuple with the EventCatalogPublished field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetEventCatalogPublishedOk() (*bool, bool) {
	if o == nil || o.EventCatalogPublished == nil {
		return nil, false
	}
	return o.EventCatalogPublished, true
}

// HasEventCatalogPublished returns a boolean if a field has been set.
func (o *SettingsOut) HasEventCatalogPublished() bool {
	if o != nil && o.EventCatalogPublished != nil {
		return true
	}

	return false
}

// SetEventCatalogPublished gets a reference to the given bool and assigns it to the EventCatalogPublished field.
func (o *SettingsOut) SetEventCatalogPublished(v bool) {
	o.EventCatalogPublished = &v
}

// GetReadOnly returns the ReadOnly field value if set, zero value otherwise.
func (o *SettingsOut) GetReadOnly() bool {
	if o == nil || o.ReadOnly == nil {
		var ret bool
		return ret
	}
	return *o.ReadOnly
}

// GetReadOnlyOk returns a tuple with the ReadOnly field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetReadOnlyOk() (*bool, bool) {
	if o == nil || o.ReadOnly == nil {
		return nil, false
	}
	return o.ReadOnly, true
}

// HasReadOnly returns a boolean if a field has been set.
func (o *SettingsOut) HasReadOnly() bool {
	if o != nil && o.ReadOnly != nil {
		return true
	}

	return false
}

// SetReadOnly gets a reference to the given bool and assigns it to the ReadOnly field.
func (o *SettingsOut) SetReadOnly(v bool) {
	o.ReadOnly = &v
}

// GetShowUseSvixPlay returns the ShowUseSvixPlay field value if set, zero value otherwise.
func (o *SettingsOut) GetShowUseSvixPlay() bool {
	if o == nil || o.ShowUseSvixPlay == nil {
		var ret bool
		return ret
	}
	return *o.ShowUseSvixPlay
}

// GetShowUseSvixPlayOk returns a tuple with the ShowUseSvixPlay field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetShowUseSvixPlayOk() (*bool, bool) {
	if o == nil || o.ShowUseSvixPlay == nil {
		return nil, false
	}
	return o.ShowUseSvixPlay, true
}

// HasShowUseSvixPlay returns a boolean if a field has been set.
func (o *SettingsOut) HasShowUseSvixPlay() bool {
	if o != nil && o.ShowUseSvixPlay != nil {
		return true
	}

	return false
}

// SetShowUseSvixPlay gets a reference to the given bool and assigns it to the ShowUseSvixPlay field.
func (o *SettingsOut) SetShowUseSvixPlay(v bool) {
	o.ShowUseSvixPlay = &v
}

// GetWipeSuccessfulPayload returns the WipeSuccessfulPayload field value if set, zero value otherwise.
func (o *SettingsOut) GetWipeSuccessfulPayload() bool {
	if o == nil || o.WipeSuccessfulPayload == nil {
		var ret bool
		return ret
	}
	return *o.WipeSuccessfulPayload
}

// GetWipeSuccessfulPayloadOk returns a tuple with the WipeSuccessfulPayload field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *SettingsOut) GetWipeSuccessfulPayloadOk() (*bool, bool) {
	if o == nil || o.WipeSuccessfulPayload == nil {
		return nil, false
	}
	return o.WipeSuccessfulPayload, true
}

// HasWipeSuccessfulPayload returns a boolean if a field has been set.
func (o *SettingsOut) HasWipeSuccessfulPayload() bool {
	if o != nil && o.WipeSuccessfulPayload != nil {
		return true
	}

	return false
}

// SetWipeSuccessfulPayload gets a reference to the given bool and assigns it to the WipeSuccessfulPayload field.
func (o *SettingsOut) SetWipeSuccessfulPayload(v bool) {
	o.WipeSuccessfulPayload = &v
}

func (o SettingsOut) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if o.ColorPaletteDark != nil {
		toSerialize["colorPaletteDark"] = o.ColorPaletteDark
	}
	if o.ColorPaletteLight != nil {
		toSerialize["colorPaletteLight"] = o.ColorPaletteLight
	}
	if o.CustomBaseFontSize.IsSet() {
		toSerialize["customBaseFontSize"] = o.CustomBaseFontSize.Get()
	}
	if o.CustomColor.IsSet() {
		toSerialize["customColor"] = o.CustomColor.Get()
	}
	if o.CustomFontFamily.IsSet() {
		toSerialize["customFontFamily"] = o.CustomFontFamily.Get()
	}
	if o.CustomFontFamilyUrl.IsSet() {
		toSerialize["customFontFamilyUrl"] = o.CustomFontFamilyUrl.Get()
	}
	if o.CustomLogoUrl.IsSet() {
		toSerialize["customLogoUrl"] = o.CustomLogoUrl.Get()
	}
	if o.CustomStringsOverride != nil {
		toSerialize["customStringsOverride"] = o.CustomStringsOverride
	}
	if o.CustomThemeOverride != nil {
		toSerialize["customThemeOverride"] = o.CustomThemeOverride
	}
	if o.DisableEndpointOnFailure != nil {
		toSerialize["disableEndpointOnFailure"] = o.DisableEndpointOnFailure
	}
	if o.DisplayName.IsSet() {
		toSerialize["displayName"] = o.DisplayName.Get()
	}
	if o.EnableChannels != nil {
		toSerialize["enableChannels"] = o.EnableChannels
	}
	if o.EnableIntegrationManagement != nil {
		toSerialize["enableIntegrationManagement"] = o.EnableIntegrationManagement
	}
	if o.EnableTransformations != nil {
		toSerialize["enableTransformations"] = o.EnableTransformations
	}
	if o.EnforceHttps != nil {
		toSerialize["enforceHttps"] = o.EnforceHttps
	}
	if o.EventCatalogPublished != nil {
		toSerialize["eventCatalogPublished"] = o.EventCatalogPublished
	}
	if o.ReadOnly != nil {
		toSerialize["readOnly"] = o.ReadOnly
	}
	if o.ShowUseSvixPlay != nil {
		toSerialize["showUseSvixPlay"] = o.ShowUseSvixPlay
	}
	if o.WipeSuccessfulPayload != nil {
		toSerialize["wipeSuccessfulPayload"] = o.WipeSuccessfulPayload
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


