/*
 * Svix API
 *
 * Welcome to the Svix API documentation!  Useful links: [Homepage](https://www.svix.com) | [Support email](mailto:support+docs@svix.com) | [Blog](https://www.svix.com/blog/) | [Slack Community](https://www.svix.com/slack/)  # Introduction  This is the reference documentation and schemas for the [Svix webhook service](https://www.svix.com) API. For tutorials and other documentation please refer to [the documentation](https://docs.svix.com).  ## Main concepts  In Svix you have four important entities you will be interacting with:  - `messages`: these are the webhooks being sent. They can have contents and a few other properties. - `application`: this is where `messages` are sent to. Usually you want to create one application for each of your users. - `endpoint`: endpoints are the URLs messages will be sent to. Each application can have multiple `endpoints` and each message sent to that application will be sent to all of them (unless they are not subscribed to the sent event type). - `event-type`: event types are identifiers denoting the type of the message being sent. Event types are primarily used to decide which events are sent to which endpoint.   ## Authentication  Get your authentication token (`AUTH_TOKEN`) from the [Svix dashboard](https://dashboard.svix.com) and use it as part of the `Authorization` header as such: `Authorization: Bearer ${AUTH_TOKEN}`.  <SecurityDefinitions />   ## Code samples  The code samples assume you already have the respective libraries installed and you know how to use them. For the latest information on how to do that, please refer to [the documentation](https://docs.svix.com/).   ## Cross-Origin Resource Sharing  This API features Cross-Origin Resource Sharing (CORS) implemented in compliance with [W3C spec](https://www.w3.org/TR/cors/). And that allows cross-domain communication from the browser. All responses have a wildcard same-origin which makes them completely public and accessible to everyone, including any code on any site. 
 *
 * API version: 1.4
 */

// Code generated by OpenAPI Generator (https://openapi-generator.tech); DO NOT EDIT.

package openapi

import (
	"encoding/json"
)

// ListResponseEndpointMessageOut struct for ListResponseEndpointMessageOut
type ListResponseEndpointMessageOut struct {
	Data []EndpointMessageOut `json:"data"`
	Done bool `json:"done"`
	Iterator *string `json:"iterator,omitempty"`
}

// NewListResponseEndpointMessageOut instantiates a new ListResponseEndpointMessageOut object
// This constructor will assign default values to properties that have it defined,
// and makes sure properties required by API are set, but the set of arguments
// will change when the set of required properties is changed
func NewListResponseEndpointMessageOut(data []EndpointMessageOut, done bool) *ListResponseEndpointMessageOut {
	this := ListResponseEndpointMessageOut{}
	this.Data = data
	this.Done = done
	return &this
}

// NewListResponseEndpointMessageOutWithDefaults instantiates a new ListResponseEndpointMessageOut object
// This constructor will only assign default values to properties that have it defined,
// but it doesn't guarantee that properties required by API are set
func NewListResponseEndpointMessageOutWithDefaults() *ListResponseEndpointMessageOut {
	this := ListResponseEndpointMessageOut{}
	return &this
}

// GetData returns the Data field value
func (o *ListResponseEndpointMessageOut) GetData() []EndpointMessageOut {
	if o == nil {
		var ret []EndpointMessageOut
		return ret
	}

	return o.Data
}

// GetDataOk returns a tuple with the Data field value
// and a boolean to check if the value has been set.
func (o *ListResponseEndpointMessageOut) GetDataOk() (*[]EndpointMessageOut, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Data, true
}

// SetData sets field value
func (o *ListResponseEndpointMessageOut) SetData(v []EndpointMessageOut) {
	o.Data = v
}

// GetDone returns the Done field value
func (o *ListResponseEndpointMessageOut) GetDone() bool {
	if o == nil {
		var ret bool
		return ret
	}

	return o.Done
}

// GetDoneOk returns a tuple with the Done field value
// and a boolean to check if the value has been set.
func (o *ListResponseEndpointMessageOut) GetDoneOk() (*bool, bool) {
	if o == nil  {
		return nil, false
	}
	return &o.Done, true
}

// SetDone sets field value
func (o *ListResponseEndpointMessageOut) SetDone(v bool) {
	o.Done = v
}

// GetIterator returns the Iterator field value if set, zero value otherwise.
func (o *ListResponseEndpointMessageOut) GetIterator() string {
	if o == nil || o.Iterator == nil {
		var ret string
		return ret
	}
	return *o.Iterator
}

// GetIteratorOk returns a tuple with the Iterator field value if set, nil otherwise
// and a boolean to check if the value has been set.
func (o *ListResponseEndpointMessageOut) GetIteratorOk() (*string, bool) {
	if o == nil || o.Iterator == nil {
		return nil, false
	}
	return o.Iterator, true
}

// HasIterator returns a boolean if a field has been set.
func (o *ListResponseEndpointMessageOut) HasIterator() bool {
	if o != nil && o.Iterator != nil {
		return true
	}

	return false
}

// SetIterator gets a reference to the given string and assigns it to the Iterator field.
func (o *ListResponseEndpointMessageOut) SetIterator(v string) {
	o.Iterator = &v
}

func (o ListResponseEndpointMessageOut) MarshalJSON() ([]byte, error) {
	toSerialize := map[string]interface{}{}
	if true {
		toSerialize["data"] = o.Data
	}
	if true {
		toSerialize["done"] = o.Done
	}
	if o.Iterator != nil {
		toSerialize["iterator"] = o.Iterator
	}
	return json.Marshal(toSerialize)
}

type NullableListResponseEndpointMessageOut struct {
	value *ListResponseEndpointMessageOut
	isSet bool
}

func (v NullableListResponseEndpointMessageOut) Get() *ListResponseEndpointMessageOut {
	return v.value
}

func (v *NullableListResponseEndpointMessageOut) Set(val *ListResponseEndpointMessageOut) {
	v.value = val
	v.isSet = true
}

func (v NullableListResponseEndpointMessageOut) IsSet() bool {
	return v.isSet
}

func (v *NullableListResponseEndpointMessageOut) Unset() {
	v.value = nil
	v.isSet = false
}

func NewNullableListResponseEndpointMessageOut(val *ListResponseEndpointMessageOut) *NullableListResponseEndpointMessageOut {
	return &NullableListResponseEndpointMessageOut{value: val, isSet: true}
}

func (v NullableListResponseEndpointMessageOut) MarshalJSON() ([]byte, error) {
	return json.Marshal(v.value)
}

func (v *NullableListResponseEndpointMessageOut) UnmarshalJSON(src []byte) error {
	v.isSet = true
	return json.Unmarshal(src, &v.value)
}


