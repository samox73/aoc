package main

import (
	"errors"
	"fmt"
	"sort"

	"github.com/cznic/mathutil"

	. "github.com/samox73/aoc/go/2022/utils"
)

func main() {
	input := GetInput(12, 2022)
	solutionA := PartA(input)
	solutionB := PartB(input)
	fmt.Printf("Day 12 solution A: %d\n", solutionA)
	fmt.Printf("Day 12 solution B: %d\n", solutionB)
}

func convertToHeight(r rune) int {
	if r == 'S' {
		return 0
	} else if r == 'E' {
		return 27
	} else {
		return int(r) - int('a') + 1
	}
}

func parseGrid(input string) *grid {
	g := &grid{}
	lines := GetLines(input)
	sizeY := len(lines)
	sizeX := len(lines[0])
	fmt.Printf("Size X: %d\nSize Y: %d\n", sizeX, sizeY)
	g.init(sizeX, sizeY)
	for y, line := range lines {
		for x, char := range line {
			// fmt.Printf("Pocessing:\n%s\n%*s\n", line, x+1, "↑")
			if char == 'S' {
				g.start = g.nodes[y][x]
			}
			if char == 'E' {
				g.end = g.nodes[y][x]
			}
			g.nodes[y][x].height = convertToHeight(char)
			g.nodes[y][x].pos = Pos2{X: x, Y: y}
		}
	}
	return g
}

func getNextNeighbor(n *node, Q map[Pos2]*node) *node {
	keys := make([]Pos2, 0, len(Q))
	for p := range Q {
		keys = append(keys, p)
	}
	sort.Slice(keys, func(i, j int) bool { return Q[keys[i]].distance < Q[keys[j]].distance })
	return Q[keys[0]]
}

func dijkstra(g *grid) (int, error) {
	Q := make(map[Pos2]*node, 0)
	for y := range g.nodes {
		for x := range g.nodes[y] {
			g.nodes[y][x].distance = mathutil.MaxInt
			g.nodes[y][x].previous = nil
			Q[g.nodes[y][x].pos] = g.nodes[y][x]
		}
	}
	g.start.distance = 0

	current := g.start
	current.visited = true
	delete(Q, current.pos)
	for len(Q) > 0 {
		for i, v := range current.neighbors {
			// fmt.Printf("  examining neighbor (%d,%d)\n", v.pos.X, v.pos.Y)
			if _, ok := Q[v.pos]; !ok {
				// fmt.Println("  neighbor does not exist in Q anymore")
				continue
			}
			newDistance := current.distance + 1
			if newDistance < v.distance {
				// fmt.Printf("  updating distance on (%d,%d) to %d\n", v.pos.X, v.pos.Y, newDistance)
				current.neighbors[i].distance = newDistance
				current.neighbors[i].previous = current
			}
		}
		nextPos := getNextNeighbor(current, Q).pos
		current = Q[nextPos]
		// fmt.Printf("Current: (%d,%d)\n", current.pos.X, current.pos.Y)
		current.visited = true
		// fmt.Printf("  deleting (%d,%d) from Q\n", nextPos.X, nextPos.Y)
		delete(Q, nextPos)
		if current.equal(*g.end) {
			break
		}
		if current == nil {
			panic(fmt.Sprintf("Could not find node with position (%d,%d)", nextPos.X, nextPos.Y))
		}
	}

	count := 0
	start := g.end
	for !start.equal(*g.start) {
		if start.previous == nil {
			return mathutil.MaxInt, errors.New("Could not find end")
		}
		start = start.previous
		if !start.equal(*g.start) {
			start.onShortestPath = true
		}
		count++
	}
	return count, nil
}

func PartA(input string) int {
	grid := parseGrid(input)
	grid.buildNeighborLists()
	count, _ := dijkstra(grid)
	grid.print()
	return count
}

func PartB(input string) int {
	// input = `SabeE`
	grid := parseGrid(input)
	grid.buildNeighborLists()
	aList := make([]*node, 0)
	for _, line := range grid.nodes {
		for _, node := range line {
			if node.height == 1 {
				aList = append(aList, node)
			}
		}
	}
	fewestCount := mathutil.MaxInt
	for _, start := range aList {
		grid.start = start
		fmt.Printf("Testing start (%d,%d)\n", start.pos.X, start.pos.Y)
		count, err := dijkstra(grid)
		fmt.Printf("  count %d\n", count)
		if err != nil {
			continue
		}
		if count < fewestCount {
			fewestCount = count
		}
		fmt.Printf("  fewest count %d\n", fewestCount)
	}
	return fewestCount
}
