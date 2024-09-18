package util

import "reflect"

// IsNilPointer exists because Golang is truly cursed: check for non-nil interface, whose underlying value is nil.
func IsNilPointer(p any) bool {
	return p == nil || reflect.ValueOf(p).IsNil()
}
