package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

func main() {
	inputPath := "../day-3-1.txt"

	input, err := os.ReadFile(inputPath)
	if err != nil {
		log.Fatalf("Failed to read input data due to error: %s", err)
	}

	nums, err := ParseValidMulData(input)
	if err != nil {
		log.Fatalf("Failed to parse valid muls with error: %s", err)
	}

	total := 0
	for _, curNum := range nums {
		total += curNum[0] * curNum[1]
	}

	fmt.Printf("Total is: %d\n", total)

	stripped, err := RemoveDontData(input)
	if err != nil {
		log.Fatalf("Remove don't data failed with error: %s", err)
	}

	nums, err = ParseValidMulData(stripped)

	total = 0
	for _, curNum := range nums {
		total += curNum[0] * curNum[1]
	}

	fmt.Printf("don't Total is: %d\n", total)
}

func ParseValidMulData(input []byte) (output [][]int, err error) {
	output = make([][]int, 0)
	mulRegex, err := regexp.Compile(`mul\((\d*),(\d*)\)`)
	if err != nil {
		return output, err
	}

	matches := mulRegex.FindAllSubmatch(input, -1)

	for _, match := range matches {
		curNums := make([]int, 2)
		curNums[0], err = strconv.Atoi(string(match[1]))
		if err != nil {
			return output, fmt.Errorf("Failed to parse num1 as a number. Got error: %s", err)
		}
		curNums[1], err = strconv.Atoi(string(match[2]))
		if err != nil {
			return output, fmt.Errorf("Failed to parse num2 as a number. Got error: %s", err)
		}

		output = append(output, curNums)
	}

	return
}

func RemoveDontData(input []byte) (output []byte, err error) {
	dontRegex, err := regexp.Compile(`don't\(\).*?do\(\)`)
	if err != nil {
		return output, err
	}

	output = dontRegex.ReplaceAll(input, []byte(""))

	return
}
