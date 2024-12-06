/*
Svix API

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

API version: 1.1.1
*/

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
	"bytes"
	"fmt"
)

// checks if the CreateMessageTokenIn type satisfies the MappedNullable interface at compile time
var _ MappedNullable = &CreateMessageTokenIn{}

// CreateMessageTokenIn struct for CreateMessageTokenIn
type CreateMessageTokenIn struct {
	// How long the token will be valid for, in seconds.
	Expiry *int32 `json:"expiry,omitempty"`
	// The name of the token.
	Name string `json:"name"`
}

type _CreateMessageTokenIn CreateMessageTokenIn

// NewCreateMessageTokenIn instantiates a new CreateMessageTokenIn object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewCreateMessageTokenIn(name string) *CreateMessageTokenIn {
	this := CreateMessageTokenIn{}
	this.Name = name
	return &this
}

// NewCreateMessageTokenInWithDefaults instantiates a new CreateMessageTokenIn object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewCreateMessageTokenInWithDefaults() *CreateMessageTokenIn {
	this := CreateMessageTokenIn{}
	return &this
}

// GetExpiry returns the Expiry field value if set, zero value otherwise.
func (o *CreateMessageTokenIn) GetExpiry() int32 {
	if o == nil || IsNil(o.Expiry) {
		var ret int32
		return ret
	}
	return *o.Expiry
}

// GetExpiryOk returns a tuple with the Expiry field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *CreateMessageTokenIn) GetExpiryOk() (*int32, bool) {
	if o == nil || IsNil(o.Expiry) {
		return nil, false
	}
	return o.Expiry, true
}

// HasExpiry returns a boolean if a field has been set.
func (o *CreateMessageTokenIn) HasExpiry() bool {
	if o != nil && !IsNil(o.Expiry) {
		return true
	}

	return false
}

// SetExpiry gets a reference to the given int32 and assigns it to the Expiry field.
func (o *CreateMessageTokenIn) SetExpiry(v int32) {
	o.Expiry = &v
}

// GetName returns the Name field value
func (o *CreateMessageTokenIn) GetName() string {
	if o == nil {
		var ret string
		return ret
	}

	return o.Name
}

// GetNameOk returns a tuple with the Name field value
// and a boolean to check if the value has been set.
func (o *CreateMessageTokenIn) GetNameOk() (*string, bool) {
	if o == nil {
		return nil, false
	}
	return &o.Name, true
}

// SetName sets field value
func (o *CreateMessageTokenIn) SetName(v string) {
	o.Name = v
}

func (o CreateMessageTokenIn) MarshalJSON() ([]byte, error) {
	toSerialize,err := o.ToMap()
	if err != nil {
		return []byte{}, err
	}
	return json.Marshal(toSerialize)
}

func (o CreateMessageTokenIn) ToMap() (map[string]interface{}, error) {
	toSerialize := map[string]interface{}{}
	if !IsNil(o.Expiry) {
		toSerialize["expiry"] = o.Expiry
	}
	toSerialize["name"] = o.Name
	return toSerialize, nil
}

func (o *CreateMessageTokenIn) UnmarshalJSON(data []byte) (err error) {
	// This validates that all required properties are included in the JSON object
	// by unmarshalling the object into a generic map with string keys and checking
	// that every required field exists as a key in the generic map.
	requiredProperties := []string{
		"name",
	}

	allProperties := make(map[string]interface{})

	err = json.Unmarshal(data, &allProperties)

	if err != nil {
		return err;
	}

	for _, requiredProperty := range(requiredProperties) {
		if _, exists := allProperties[requiredProperty]; !exists {
			return fmt.Errorf("no value given for required property %v", requiredProperty)
		}
	}

	varCreateMessageTokenIn := _CreateMessageTokenIn{}

	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	err = decoder.Decode(&varCreateMessageTokenIn)

	if err != nil {
		return err
	}

	*o = CreateMessageTokenIn(varCreateMessageTokenIn)

	return err
}

type NullableCreateMessageTokenIn struct {
	value *CreateMessageTokenIn
	isSet bool
}

func (v NullableCreateMessageTokenIn) Get() *CreateMessageTokenIn {
	return v.value
}

func (v *NullableCreateMessageTokenIn) Set(val *CreateMessageTokenIn) {
	v.value = val
	v.isSet = true
}

func (v NullableCreateMessageTokenIn) IsSet() bool {
	return v.isSet
}

func (v *NullableCreateMessageTokenIn) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableCreateMessageTokenIn(val *CreateMessageTokenIn) *NullableCreateMessageTokenIn {
	return &NullableCreateMessageTokenIn{value: val, isSet: true}
}

func (v NullableCreateMessageTokenIn) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableCreateMessageTokenIn) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


