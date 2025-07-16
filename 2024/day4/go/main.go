package main

import (
	"fmt"
	"log"
)

func main() {
	inputPath := "../day-4-1.txt"
	data, err := readData(inputPath)
	if err != nil {
		log.Fatalf("Failed to read data. Got error: %s", err)
	}

	xmas := findXmas(data)
	fmt.Printf("Found %d xmas\n", xmas)
}
