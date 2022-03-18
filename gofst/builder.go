package main

import (
	"fmt"
	"reflect"
	"unsafe"
)

/*
#cgo CFLAGS: -I.
#cgo LDFLAGS: -L../target/debug -lfurze
#include "ffi.h"
*/
import "C"

type Builder struct {
	fstBuilder unsafe.Pointer
}

func (b *Builder) Add(k []byte, v uint64) error {
	res := C.add_key(b.fstBuilder, (*C.uchar)(unsafe.Pointer(&k[0])), C.uint(len(k)), C.ulong(v))
	if res == -1 {
		return nil
	}
	return nil
}

func (b *Builder) Bytes() []byte {
	var length, capacity uint32
	bytes := C.bytes(b.fstBuilder, (*C.uint)(unsafe.Pointer(&length)), (*C.uint)(unsafe.Pointer(&capacity)))
	var data []byte
	h := (*reflect.SliceHeader)((unsafe.Pointer(&data)))
	h.Data = uintptr(unsafe.Pointer(bytes))
	h.Len = int(length)
	h.Cap = int(capacity)
	return data
}

func New() *Builder {
	return &Builder{
		fstBuilder: C.new_fst_builder(),
	}
}

type FST struct {
}

func Load(b []byte) *FST {

}

func main() {
	b := New()
	b.Add([]byte("aa"), 1)
	b.Add([]byte("bb"), 2)
	b.Add([]byte("cc"), 3)
	fmt.Println(b.Bytes())
}
