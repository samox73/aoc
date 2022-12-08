package main

import (
	"fmt"
	"strings"

	"github.com/samox73/aoc/utils"
)

func main() {
	input := utils.GetInput(4, 2022)
	fmt.Println(input)
	// fmt.Printf("Day 04 solution: %d", partA(input))
}

type assignment struct {
	from, to int64
}

func covered(x, y assignment) bool {
	return x.from <= y.from && x.to >= y.to || x.from >= y.from && x.to <= y.to
}

func partA(input string) int {
	s := strings.Split(input, "\n")
	count := 0
	for _, line := range s {
		fmt.Println(line)
		leftElf, rightElf := utils.SplitPair(line, ",")
		leftElfBegin, leftElfEnd := utils.SplitIntPair(leftElf, "-")
		rightElfBegin, rightElfEnd := utils.SplitIntPair(rightElf, "-")
		leftAssignment := assignment{from: leftElfBegin, to: leftElfEnd}
		rightAssignment := assignment{from: rightElfBegin, to: rightElfEnd}
		if covered(leftAssignment, rightAssignment) {
			count += 1
		}
	}
	return count
}
