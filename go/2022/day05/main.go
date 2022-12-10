package main

import (
	"fmt"
	"regexp"
	"strconv"
	"strings"

	"github.com/samox73/aoc/go/2022/utils"
)

func main() {
	input := utils.GetInput(5, 2022)
	solutionA := PartA(input)
	solutionB := PartB(input)
	fmt.Printf("Day 05 solution A: %s\n", solutionA)
	fmt.Printf("Day 05 solution B: %s\n", solutionB)
}

func PartB(input string) string {
	stacks := GetStacks(input)
	instructions := GetInstructions(input)
	for _, instruction := range instructions {
		stackFrom := &stacks[instruction.from]
		stackTo := &stacks[instruction.to]
		crates := make([]string, instruction.count)
		for i := int64(0); i < instruction.count; i++ {
			crates[i] = stackFrom.Pop()
		}
		for i := instruction.count - 1; i > int64(-1); i-- {
			stackTo.Put(crates[i])
		}
	}
	result := ""
	for _, stack := range stacks {
		crate := strings.ReplaceAll(strings.ReplaceAll(stack.Pop(), "[", ""), "]", "")
		result = result + crate
	}
	return result
}

func PartA(input string) string {
	stacks := GetStacks(input)
	instructions := GetInstructions(input)
	for _, instruction := range instructions {
		stackFrom := &stacks[instruction.from]
		stackTo := &stacks[instruction.to]
		for i := int64(0); i < instruction.count; i++ {
			crate := stackFrom.Pop()
			stackTo.Put(crate)
		}
	}
	result := ""
	for _, stack := range stacks {
		crate := strings.ReplaceAll(strings.ReplaceAll(stack.Pop(), "[", ""), "]", "")
		result = result + crate
	}
	return result
}

func GetInstructions(input string) []Instruction {
	floorIdx := GetFloor(input)
	lines := utils.GetLines(input)
	re := regexp.MustCompile(`\d+`)
	var instructions []Instruction
	for i := floorIdx + 2; i < len(lines); i++ {
		matches := utils.ToInts(re.FindAllString(lines[i], -1))
		instruction := Instruction{count: matches[0], from: matches[1] - 1, to: matches[2] - 1}
		instructions = append(instructions, instruction)
	}
	return instructions
}

func GetFloor(input string) int {
	lines := utils.GetLines(input)
	for idx, line := range lines {
		lineArray := strings.Fields(line)
		if len(lineArray) == 0 {
			continue
		}
		lineIsFloor := true
		for _, s := range lineArray {
			_, err := strconv.Atoi(s)
			if err != nil {
				lineIsFloor = false
				continue
			}
		}
		if lineIsFloor {
			return idx
		}
	}
	return -1
}

func GetStacks(input string) []Stack {
	floorIndex := GetFloor(input)
	lines := utils.GetLines(input)
	floor := strings.Fields(lines[floorIndex])
	stacksCount := len(floor)
	var stacks []Stack
	for stackIdx := 0; stackIdx < stacksCount; stackIdx++ {
		stacks = append(stacks, Stack{})
		for crateIdx := floorIndex - 1; crateIdx > -1; crateIdx-- {
			line := strings.Split(lines[crateIdx], "")
			crate := strings.Join(line[stackIdx*4:(stackIdx+1)*4-1], "")
			if len(strings.TrimSpace(crate)) != 0 {
				stacks[stackIdx].Put(crate)
			}
		}
	}
	return stacks
}
