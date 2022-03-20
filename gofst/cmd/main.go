package main

import (
	"fmt"
	"gofst"
)

func main() {
	b := gofst.NewBuilder()
	b.Add([]byte("aa"), 1)
	b.Add([]byte("bb"), 2)
	b.Add([]byte("cc"), 3)
	b.Add([]byte("yun"), 9)
	b.Add([]byte("zzzz"), 10)
	b.Finish()
	fmt.Println(b.Bytes())

	f := gofst.Load(b.Bytes())
	fmt.Println(f.Get([]byte("cc")))

}
