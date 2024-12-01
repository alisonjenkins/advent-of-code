package main

import (
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
)

func main() {
	day1P1InputPath := "../day1-1.txt"
	input, err := os.ReadFile(day1P1InputPath)
	if err != nil {
		log.Fatalf("Expected to be able to read the data input file but got error: %s", err)
	}

	output, err := day1p1(input)
	if err != nil {
		log.Fatalf("Failed to process input. Got error: %s", err)
	}

	fmt.Printf("Output:\n%d\n", output)
}

func day1p1(data []byte) (output int, err error) {
	entryLength := findEntryLength(data)
	list1, list2, records, err := parseRecords(entryLength, data)
	if err != nil {
		log.Fatalf("Failed to parse records with error: %s", err)
	}

	totalDistance := 0

	for curRecord := 0; curRecord < records; curRecord++ {
		totalDistance += findDistance(list1[curRecord], list2[curRecord])
	}

	return totalDistance, err
}

func findDistance(entry1, entry2 int) int {
	if entry1 > entry2 {
		return entry1 - entry2
	}

	return entry2 - entry1
}

func findEntryLength(input []byte) (entryLength int) {
	// count bytes from beginning of file to newline
	rowLength := 0
	for _, curChar := range string(input) {
		rowLength += 1
		if curChar == '\n' {
			break
		}
	}

	entryLength = (rowLength - 3) / 2
	return
}

func parseRecords(entryLength int, input []byte) (list1, list2 []int, records int, err error) {
	dataLength := len(input)
	dataEntries := (dataLength / ((entryLength * 2) + 4))

	list1 = make([]int, dataEntries)
	list2 = make([]int, dataEntries)

	offset := 0

	for offset < dataLength {
		list1[records], err = strconv.Atoi(string(input[offset : offset+entryLength]))
		if err != nil {
			err = fmt.Errorf("Failed to parse list 1 entry as an integer due to error: %w", err)
			return
		}

		entry2Start := offset + entryLength + 3
		entry2End := (offset + (entryLength * 2) + 3)
		list2[records], err = strconv.Atoi(string(input[entry2Start:entry2End]))
		if err != nil {
			err = fmt.Errorf("Failed to parse list 2 entry as an integer due to error: %w", err)
			return
		}

		offset += (entryLength * 2) + 4
		records += 1
	}

	slices.Sort(list1)
	slices.Sort(list2)

	return
}
