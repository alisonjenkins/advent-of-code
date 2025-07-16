package main

import "testing"

func TestReadData(t *testing.T) {
	inputPath := "../day-4-1-test-data.txt"
	data, err := readData(inputPath)
	if err != nil {
		t.Fatalf("Failed to read data. Got error: %s", err)
	}

	if len(data) != 10 {
		t.Fatalf("Expected read data to be 10 lines instead got: %d", len(data))
	}

	if len(data[0]) != 10 {
		t.Fatalf("Expected the first line to have 10 characters instead got %d", len(data[0]))
	}

	if len(data[9]) != 10 {
		t.Fatalf("Expected the last line to have 10 characters instead got %d", len(data[9]))
	}
}
