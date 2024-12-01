package main

import (
	"os"
	"testing"
)

func TestFindRowLength(t *testing.T) {
	t.Run("Works for test data", func(t *testing.T) {
		testDataPath := "../day1-1-test-data.txt"

		input, err := os.ReadFile(testDataPath)
		if err != nil {
			t.Fatalf("Expected to be able to read the test data input file but got error: %s", err)
		}

		expectedLength := 1
		length := findEntryLength(input)

		if length != expectedLength {
			t.Fatalf(
				"Expected to get length: %d with test data but got length %d",
				expectedLength,
				length,
			)
		}
	})

	t.Run("Works for real data", func(t *testing.T) {
		testDataPath := "../day1-1.txt"

		input, err := os.ReadFile(testDataPath)
		if err != nil {
			t.Fatalf("Expected to be able to read the test data input file but got error: %s", err)
		}

		expectedLength := 5
		length := findEntryLength(input)

		if length != expectedLength {
			t.Fatalf(
				"Expected to get length: %d with real data but got length %d",
				expectedLength,
				length,
			)
		}
	})
}

func TestParseRecords(t *testing.T) {
	t.Run("Works for test data", func(t *testing.T) {
		testDataPath := "../day1-1-test-data.txt"

		input, err := os.ReadFile(testDataPath)
		if err != nil {
			t.Fatalf("Expected to be able to read the test data input file but got error: %s", err)
		}

		entryLength := findEntryLength(input)

		list1, list2, records, err := parseRecords(entryLength, input)
		if err != nil {
			t.Fatalf("Expected to not fail to parse records with test data but got error: %s", err)
		}

		expectedList1 := []int{
			1,
			2,
			3,
			3,
			3,
			4,
		}
		expectedList2 := []int{
			3,
			3,
			3,
			4,
			5,
			9,
		}
		expectedRecords := 6

		for entryNum := range expectedList1 {
			if expectedList1[entryNum] != list1[entryNum] {
				t.Fatalf(
					"Expected list 1 entry %d to match expected value %d but got %d",
					entryNum,
					expectedList1[entryNum],
					list1[entryNum],
				)
			}

			if expectedList2[entryNum] != list2[entryNum] {
				t.Fatalf(
					"Expected list 2 entry %d to match expected value %d but got %d",
					entryNum,
					expectedList2[entryNum],
					list2[entryNum],
				)
			}
		}

		if records != expectedRecords {
			t.Fatalf("Expected to get %d records but got %d", expectedRecords, records)
		}
	})

	t.Run("Works for real data", func(t *testing.T) {
		testDataPath := "../day1-1.txt"

		input, err := os.ReadFile(testDataPath)
		if err != nil {
			t.Fatalf("Expected to be able to read the real data input file but got error: %s", err)
		}

		entryLength := findEntryLength(input)

		list1, list2, records, err := parseRecords(entryLength, input)
		if err != nil {
			t.Fatalf("Expected to not fail to parse records with real data but got error: %s", err)
		}

		expectedList1 := []int{
			10182,
			10336,
		}
		expectedList2 := []int{
			10108,
			10498,
		}
		expectedRecords := 1000

		for entryNum := range expectedList1 {
			if expectedList1[entryNum] != list1[entryNum] {
				t.Fatalf(
					"Expected list 1 entry %d to match expected value %d but got %d",
					entryNum,
					expectedList1[entryNum],
					list1[entryNum],
				)
			}

			if expectedList2[entryNum] != list2[entryNum] {
				t.Fatalf(
					"Expected list 2 entry %d to match expected value %d but got %d",
					entryNum,
					expectedList2[entryNum],
					list2[entryNum],
				)
			}
		}

		if records != expectedRecords {
			t.Fatalf("Expected to get %d records but got %d", expectedRecords, records)
		}
	})
}

func TestFindDistance(t *testing.T) {
	expected := 2
	actual := findDistance(1, 3)
	if actual != expected {
		t.Fatalf("Expected to get '%d' as output but got '%d'", expected, actual)
	}

	expected = 3
	actual = findDistance(4, 1)
	if actual != expected {
		t.Fatalf("Expected to get '%d' as output but got '%d'", expected, actual)
	}
}
