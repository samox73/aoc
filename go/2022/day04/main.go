package main

import (
	"fmt"
	"strings"

	"github.com/samox73/aoc/go/2022/utils"
)

func main() {
	input := utils.GetInput(4, 2022)
	fmt.Println(input)
	solutionA, solutionB := solution(input)
	fmt.Printf("Day 04 solution A: %d\n", solutionA)
	fmt.Printf("Day 04 solution B: %d\n", solutionB)
}

type assignment struct {
	from, to int
}

func FullyRedundant(x, y assignment) bool {
	return x.from <= y.from && x.to >= y.to || x.from >= y.from && x.to <= y.to
}

func PartiallyRedundant(x, y assignment) bool {
	return x.from >= y.from && x.from <= y.to || y.from >= x.from && y.from <= x.to
}

func solution(input string) (int, int) {
	s := strings.Split(input, "\n")
	countA := 0
	countB := 0
	for _, line := range s {
		fmt.Println(line)
		leftElf, rightElf := utils.SplitPair(line, ",")
		leftElfBegin, leftElfEnd := utils.SplitIntPair(leftElf, "-")
		rightElfBegin, rightElfEnd := utils.SplitIntPair(rightElf, "-")
		leftAssignment := assignment{from: leftElfBegin, to: leftElfEnd}
		rightAssignment := assignment{from: rightElfBegin, to: rightElfEnd}
		if FullyRedundant(leftAssignment, rightAssignment) {
			countA += 1
		}
		if PartiallyRedundant(leftAssignment, rightAssignment) {
			countB += 1
		}
	}
	return countA, countB
}
