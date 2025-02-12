package utils

import "encoding/json"

type Nullable[T any] struct {
	val   *T
	isSet bool
}

// Create a new Nullable[T] that is unset, This type will not be emitted when serializing
func NewUnsetNullable[T any]() Nullable[T] {
	return Nullable[T]{val: nil, isSet: false}
}

// Create a new Nullable[T] from a pointer to a value
//
// If you need to create an Nullable[string] that is explicitly set to nil, use this method
func NewNullableFromPtr[T any](value *T) Nullable[T] {
	return Nullable[T]{val: value, isSet: true}
}

// Create a new Nullable[T] from a value
func NewNullable[T any](value T) Nullable[T] {
	return Nullable[T]{val: &value, isSet: true}
}

func (v Nullable[T]) Get() *T {
	return v.val
}

func (v *Nullable[T]) Set(val *T) {
	v.val = val
	v.isSet = true
}

func (v Nullable[T]) IsSet() bool {
	return v.isSet
}

func (v *Nullable[T]) Unset() {
	v.val = nil
	v.isSet = false
}

func (n *Nullable[T]) UnmarshalJSON(data []byte) error {
	n.isSet = true
	if string(data) == "null" {
		return nil
	}

	var value T
	if err := json.Unmarshal(data, &value); err != nil {
		return err
	}

	n.val = &value
	return nil
}

func (n Nullable[T]) MarshalJSON() ([]byte, error) {
	if n.isSet {
		return json.Marshal(n.val)
	}
	return nil, nil

}
