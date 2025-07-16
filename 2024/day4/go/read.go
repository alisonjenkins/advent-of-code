package main

import (
	"bufio"
	"fmt"
	"os"
)

func readData(path string) (output []string, err error) {
	output = make([]string, 0)

	file, err := os.Open(path)
	if err != nil {
		return output, fmt.Errorf("Failed to open file: %s. Got error: %w", path, err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		output = append(output, scanner.Text())
	}
	if scanner.Err() != nil {
		return output, fmt.Errorf("scan file error: %w", scanner.Err())
	}

	return
}
