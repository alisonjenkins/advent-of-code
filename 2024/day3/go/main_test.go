package main

import (
	"io/fs"
	"testing"
)

func TestParse(t *testing.T) {
	inputPath := "../day-3-1-test-data.txt"
	input, err = fs.ReadFile()
	if err != nil {
		t.Fatalf("Failed to read input data due to error: %s", err)
	}

	muls, err := ParseValidMulData(input)
	if err != nil {
		t.Fatalf("Failed to parse valid mul data due to error: %s", err)
	}
}
