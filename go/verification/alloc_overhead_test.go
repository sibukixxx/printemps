package main

import (
	"runtime"
	"testing"
)

func BenchmarkMakeContainer(b *testing.B) {
	for i := 0; i < b.N; i++ {
		runtime.KeepAlive(MakeContainer())
	}
}

func BenchmarkMakeContainerOneLine(b *testing.B) {
	for i := 0; i < b.N; i++ {
		runtime.KeepAlive(MakeContainerOneLine())
	}
}

func BenchmarkMakeContainerNew(b *testing.B) {
	for i := 0; i < b.N; i++ {
		runtime.KeepAlive(MakeContainerNew())
	}
}
