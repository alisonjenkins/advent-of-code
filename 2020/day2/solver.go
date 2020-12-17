package main

import (
	"bufio"
	"log"
	"os"
	"regexp"
	"strconv"
)

func sledPasswords() (valid int) {
	file, err := os.Open("puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}

	passRegex := regexp.MustCompile(`(?P<min>\d*)-(?P<max>\d*) (?P<policyCharacter>\w): (?P<password>\w*)`)
	passRegexNames := passRegex.SubexpNames()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		result := passRegex.FindAllStringSubmatch(line, -1)
		groups := map[string]string{}
		for nameIndex, match := range result[0] {
			groups[passRegexNames[nameIndex]] = match
		}

		min, err := strconv.Atoi(groups["min"])
		if err != nil {
			log.Printf("%s is not a min number on line '%s'", groups["min"], line)
		}

		max, err := strconv.Atoi(groups["max"])
		if err != nil {
			log.Printf("%s is not a max number on line '%s'", groups["max"], line)
		}

		policyCharacter := rune(groups["policyCharacter"][0])
		password := groups["password"]
		occurances := 0

		for _, char := range password {
			if char == policyCharacter {
				occurances++
			}
		}

		log.Printf("min: %d, max: %d, char: %s, pass: %s", min, max, string(policyCharacter), password)

		if occurances >= min && occurances <= max {
			valid++
		}
	}

	return valid
}

func tobogganPasswords() (valid int) {
	file, err := os.Open("puzzle_input.txt")
	if err != nil {
		log.Fatal(err)
	}

	passRegex := regexp.MustCompile(`(?P<pos1>\d*)-(?P<pos2>\d*) (?P<policyCharacter>\w): (?P<password>\w*)`)
	passRegexNames := passRegex.SubexpNames()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := scanner.Text()

		result := passRegex.FindAllStringSubmatch(line, -1)
		groups := map[string]string{}
		for nameIndex, match := range result[0] {
			groups[passRegexNames[nameIndex]] = match
		}

		pos1, err := strconv.Atoi(groups["pos1"])
		if err != nil {
			log.Printf("%s is not a pos1 number on line '%s'", groups["pos1"], line)
		}

		pos2, err := strconv.Atoi(groups["pos2"])
		if err != nil {
			log.Printf("%s is not a pos2 number on line '%s'", groups["pos2"], line)
		}

		policyCharacter := rune(groups["policyCharacter"][0])
		password := groups["password"]

		pos1Match := rune(password[pos1-1]) == policyCharacter
		pos2Match := rune(password[pos2-1]) == policyCharacter

		log.Printf("pos1: %d, pos2: %d, char: %s, pass: %s", pos1, pos2, string(policyCharacter), password)

		if (pos1Match || pos2Match) && !(pos1Match && pos2Match) {
			valid++
		}
	}

	return valid
}

func main() {
	log.Print("Started")
	sledValid := sledPasswords()
	log.Printf("The sled answer is %d", sledValid)
	tobogganValid := tobogganPasswords()
	log.Printf("The toboggan answer is %d", tobogganValid)
}
