package main

import "testing"

func TestFindXmasDir(t *testing.T) {
	inputPath := "../day-4-1-test-data.txt"
	data, err := readData(inputPath)
	if err != nil {
		t.Fatalf("Failed to read data. Got error: %s", err)
	}

	row := 0
	col := 0

	if findXmasDir(data, row, col, "up") {
		t.Fatalf("Expected to not find xmas up at: %d, %d", row, col)
	}

	row = 9
	col = 9
	if !findXmasDir(data, row, col, "up") {
		t.Fatalf("Expected to find xmas up at: %d, %d", row, col)
	}

	row = 0
	col = 0

	if findXmasDir(data, row, col, "down") {
		t.Fatalf("Expected to not find xmas down at: %d, %d", row, col)
	}

	row = 3
	col = 9

	if !findXmasDir(data, row, col, "down") {
		t.Fatalf("Expected to find xmas down at: %d, %d", row, col)
	}

	row = 9
	col = 9

	if findXmasDir(data, row, col, "down") {
		t.Fatalf("Expected to not find xmas down at: %d, %d", row, col)
	}

	row = 9
	col = 9
	if findXmasDir(data, row, col, "down") {
		t.Fatalf("Expected to not find xmas down at: %d, %d", row, col)
	}

	row = 1
	col = 4
	if !findXmasDir(data, row, col, "left") {
		t.Fatalf("Expected to find xmas down at: %d, %d", row, col)
	}

	row = 0
	col = 0
	if findXmasDir(data, row, col, "left") {
		t.Fatalf("Expected to not find xmas down at: %d, %d", row, col)
	}

	row = 4
	col = 0
	if !findXmasDir(data, row, col, "right") {
		t.Fatalf("Expected to find xmas right at: %d, %d", row, col)
	}

	row = 5
	col = 6
	if !findXmasDir(data, row, col, "ul") {
		t.Fatalf("Expected to find xmas ul at: %d, %d", row, col)
	}

	row = 5
	col = 6
	if !findXmasDir(data, row, col, "ul") {
		t.Fatalf("Expected to find xmas ul at: %d, %d", row, col)
	}

	row = 5
	col = 5
	if findXmasDir(data, row, col, "ul") {
		t.Fatalf("Expected to not find xmas ul at: %d, %d", row, col)
	}

	row = 5
	col = 0
	if !findXmasDir(data, row, col, "ur") {
		t.Fatalf("Expected to not find xmas ul at: %d, %d", row, col)
	}

	row = 0
	col = 0
	if findXmasDir(data, row, col, "dl") {
		t.Fatalf("Expected to not find xmas dl at: %d, %d", row, col)
	}

	row = 3
	col = 9
	if !findXmasDir(data, row, col, "dl") {
		t.Fatalf("Expected to find xmas dl at: %d, %d", row, col)
	}

	row = 0
	col = 4
	if !findXmasDir(data, row, col, "dr") {
		t.Fatalf("Expected to find xmas dr at: %d, %d", row, col)
	}
}

func TestFindXmas(t *testing.T) {
	inputPath := "../day-4-1-test-data.txt"
	data, err := readData(inputPath)
	if err != nil {
		t.Fatalf("Failed to read data. Got error: %s", err)
	}

	xmas := findXmas(data)
	expectedXmas := 18

	if xmas != expectedXmas {
		t.Fatalf("Expected %d xmas but got %d", expectedXmas, xmas)
	}
}
