package main

import (
	"fmt"

	. "github.com/samox73/aoc/go/2022/utils"
)

func main() {
	input := GetInput(12, 2022)
	solutionA := PartA(input)
	solutionB := PartB(input)
	fmt.Printf("Day 12 solution A: %d\n", solutionA)
	fmt.Printf("Day 12 solution B: %d\n", solutionB)
}

type node struct {
	visited       bool
	pos           Pos2
	height        float32
	neighbors     []*node
	possibleMoves []Vec2
}

type grid struct {
	nodes [][]node
}

func (g *grid) init(x, y int) {
	g.nodes = make([][]node, y)
	for i := range g.nodes {
		g.nodes[i] = make([]node, x)
	}
}

func convertToHeight(r rune) int {
	return int(r) - int('a')
}

func generateGrid(input string) grid {
	g := grid{}
	lines := GetLines(input)
	sizeX := len(lines)
	sizeY := len(lines[0])
	g.init(sizeX, sizeY)
	// for _, line := range lines {

	// }
	return g
}

func PartA(input string) int {

	return 0
}

func PartB(input string) int {
	return 0
}
