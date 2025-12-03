package main

import (
	"testing"
)

func TestProcessSteps(t *testing.T) {
	// --- Step 1 ---
	got1 := RunStep1("test.txt")
	want1 := 3

	if got1 != want1 {
		t.Errorf("RunStep1() = %d, want %d", got1, want1)
	}

	// --- Step 2 ---
	got2 := RunStep2("test.txt")
	want2 := 6

	if got2 != want2 {
		t.Errorf("RunStep2() = %d, want %d", got2, want2)
	}
}
