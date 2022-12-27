package main

import (
	"fmt"
	"strings"

	"github.com/samox73/aoc/go/2022/utils"
)

func main() {
	input := utils.GetInput(10, 2022)
	solutionA := PartA(input)
	solutionB := PartB(input)
	fmt.Printf("Day 10 solution A: %d\n", solutionA)
	fmt.Printf("Day 10 solution B:\n%s\n", solutionB)
}

type instruction struct {
	duration int
	f        func(int) int
}

type cpu struct {
	x            int
	cycle        int
	instructions map[string]instruction
}

func (c *cpu) getInstruction(name string) instruction {
	return c.instructions[name]
}

func PartA(input string) int {
	noop := instruction{duration: 1, f: func(v int) int { return 0 }}
	addx := instruction{duration: 2, f: func(v int) int { return v }}
	cpu := cpu{x: 1, cycle: 1, instructions: map[string]instruction{"noop": noop, "addx": addx}}
	solution := 0
	for _, line := range utils.GetLines(input) {
		for name, instruction := range cpu.instructions {
			activated := false
			tokens := strings.Split(line, " ")
			v := 0
			if tokens[0] == name {
				for ; instruction.duration > 0; instruction.duration-- {
					fmt.Printf("\nCycle: %d\nX:     %d\nOp:    %s\n", cpu.cycle, cpu.x, line)
					if len(tokens) > 1 {
						v = utils.ToInt(tokens[1])
					}
					if (cpu.cycle-20)%40 == 0 {
						strength := cpu.cycle * cpu.x
						fmt.Printf("Strength: %d\n", strength)
						solution += strength
					}
					if !activated && instruction.duration == 1 {
						activated = true
						cpu.x += instruction.f(v)
					}
					cpu.cycle++
				}
			}
		}
	}

	return solution
}

func spritePos(pos int) string {
	sol := ""
	for i := 0; i < 40; i++ {
		if utils.AbsInt(pos-i) <= 1 {
			sol += "#"
		} else {
			sol += "."
		}
	}
	return sol
}

func PartB(input string) string {
	noop := instruction{duration: 1, f: func(v int) int { return 0 }}
	addx := instruction{duration: 2, f: func(v int) int { return v }}
	cpu := cpu{x: 1, cycle: 1, instructions: map[string]instruction{"noop": noop, "addx": addx}}
	solution := ""
	for _, line := range utils.GetLines(input) {
		for name, instruction := range cpu.instructions {
			activated := false
			tokens := strings.Split(line, " ")
			v := 0
			if tokens[0] == name {
				for ; instruction.duration > 0; instruction.duration-- {
					if utils.AbsInt(cpu.x+1-(cpu.cycle%40)) <= 1 {
						solution += "#"
					} else {
						solution += "."
					}
					if (len(strings.ReplaceAll(solution, "\n", "")))%40 == 0 {
						solution += "\n"
					}
					fmt.Printf("\nSprite row: %s\n", spritePos(cpu.x))
					fmt.Printf("Cycle: %d\nX:     %d\nOp:    %s\n", cpu.cycle, cpu.x, line)
					if len(tokens) > 1 {
						v = utils.ToInt(tokens[1])
					}
					if !activated && instruction.duration == 1 {
						activated = true
						cpu.x += instruction.f(v)
					}
					cpu.cycle++
				}
			}
		}
	}

	return solution
}
