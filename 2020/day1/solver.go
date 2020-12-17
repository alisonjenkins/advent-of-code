package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func readNumbers() []int {
	file, err := os.Open("puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(file)
	result := []int{}

	for scanner.Scan() {
		line := scanner.Text()

		lineNum, err := strconv.Atoi(line)
		if err != nil {
			log.Printf("%s is not a number", line)
			continue
		}
		result = append(result, lineNum)
	}

	return result
}

func find2020(numbers []int) int {
	goal := 2020
	answer := 0
	for _, a := range numbers {
		for _, b := range numbers {
			if (a + b) == goal {
				log.Printf("The numbers are: %d and %d", a, b)
				answer = a * b
			}
		}
		if answer != 0 {
			break
		}
	}
	return answer
}

func find2020p2(numbers []int) int {
	goal := 2020
	answer := 0
	for _, a := range numbers {
		for _, b := range numbers {
			for _, c := range numbers {
				if (a + b + c) == goal {
					log.Printf("The numbers are: %d, %d and %d", a, b, c)
					answer = a * b * c
				}
			}
			if answer != 0 {
				break
			}
		}
	}
	return answer
}

func main() {
	log.Print("Started")
	numbers := readNumbers()
	answer := find2020(numbers)
	log.Printf("The answer is %d", answer)
	answer = find2020p2(numbers)
	log.Printf("The part 2 answer is %d", answer)
}
