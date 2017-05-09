// alloc_overhead.go

package main

type container struct {
	v [64]byte
}

func MakeContainer() *container {
	c := container{}
	return &c
}

func MakeContainerOneLine() *container {
	return &container{}
}

func MakeContainerNew() *container {
	return new(container)
}

func main() {
	_ = MakeContainer()
	_ = MakeContainerOneLine()
	_ = MakeContainerNew()
}
