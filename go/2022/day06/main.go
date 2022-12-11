package main

import (
	"fmt"
	"strings"

	"github.com/samox73/aoc/go/2022/utils"
)

func main() {
	input := utils.GetInput(6, 2022)
	solutionA := PartA(input)
	solutionB := PartB(input)
	fmt.Printf("Day 06 solution A: %d\n", solutionA)
	fmt.Printf("Day 06 solution B: %d\n", solutionB)
}

func PartA(input string) int {
	markers := MovingQueue{markers: make([]string, 4), size: 0, maxSize: 4}
	for idx, marker := range strings.Split(input, "") {
		markers.add(marker)
		markers.print()
		if markers.unique() {
			return idx + 1
		}
	}
	return -1
}

func PartB(input string) int {
	markers := MovingQueue{markers: make([]string, 14), size: 0, maxSize: 14}
	for idx, marker := range strings.Split(input, "") {
		markers.add(marker)
		markers.print()
		if markers.unique() {
			return idx + 1
		}
	}
	return -1
}
