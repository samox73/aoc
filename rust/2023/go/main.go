package main

import (
	"fmt"
	"log"
	"regexp"
	"strconv"
	"strings"

	aocdownloader "github.com/czyber/aoc-downloader"
)

func readInput() []string {
	input, err := aocdownloader.GetInput("2023", "1")
	if err != nil {
		log.Fatal("Error getting input: ", err)
	}

	inputStrings := strings.Split(input, "\n")

	return inputStrings
}

func part1() {
	inputStrings := readInput()

	re := regexp.MustCompile(`\d`)

	sum := 0
	for _, line := range inputStrings {
		numbers := re.FindAllString(line, -1)
		if len(numbers) > 0 {
			firstNumber := numbers[0]
			secondNumber := numbers[len(numbers)-1]
			integer, err := strconv.ParseInt(firstNumber+secondNumber, 10, 64)
			if err != nil {
				log.Fatal("Error parsing integer: ", err)
			}
			sum += int(integer)
		}
	}
	fmt.Println("Result: ", sum)
}

func part2() {
	inputStrings := readInput()

	re := regexp.MustCompile(`one|two|three|four|five|six|seven|eight|nine|\d`)
	wordToNumber := map[string]string{
		"one":   "1",
		"two":   "2",
		"three": "3",
		"four":  "4",
		"five":  "5",
		"six":   "6",
		"seven": "7",
		"eight": "8",
		"nine":  "9",
	}
	sum := 0
	for _, line := range inputStrings {
		numbers := re.FindAllString(line, -1)
		numbersNumeric := []string{}
		if len(numbers) > 0 {
			for _, number := range numbers {
				if wordToNumber[number] != "" {
					numbersNumeric = append(numbersNumeric, wordToNumber[number])
				} else {
					numbersNumeric = append(numbersNumeric, number)
				}
			}
			firstNumber := numbersNumeric[0]
			secondNumber := numbersNumeric[len(numbersNumeric)-1]
			integer, err := strconv.ParseInt(firstNumber+secondNumber, 10, 64)
			if err != nil {
				log.Fatal("Error parsing integer: ", err)
			}
			sum += int(integer)
		}
	}
	fmt.Println("Result: ", sum)

}

func main() {

	// Part 1
	part1()
	// Part 2
	part2()
}
