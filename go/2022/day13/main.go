package main

import (
	"fmt"

	. "github.com/samox73/aoc/go/2022/utils"
)

func main() {
	input := GetInput(13, 2022)
	solutionA := PartA(input)
	solutionB := PartB(input)
	fmt.Printf("Day 13 solution A: %d\n", solutionA)
	fmt.Printf("Day 13 solution B: %d\n", solutionB)
}

type packet []interface{}

func PartA(input string) int {
	fmt.Println(input)
	p := packet{}
	for _, line := range GetLines(input) {
		
	}
	return 0
}

func PartB(input string) int {
	return 0
}
