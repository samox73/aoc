package main

import (
	"fmt"

	. "github.com/samox73/aoc/go/2022/utils"
)

type node struct {
	visited        bool
	pos            Pos2
	distance       int
	height         int
	neighbors      map[Vec2]*node
	previous       *node
	onShortestPath bool
}

type grid struct {
	start *node
	end   *node
	nodes [][]*node
}

func (a node) equal(b node) bool {
	return a.pos.Equals(b.pos)
}

func (g *grid) init(xMax, yMax int) {
	g.nodes = make([][]*node, yMax)
	for y := range g.nodes {
		g.nodes[y] = make([]*node, xMax)
		for x := range g.nodes[y] {
			g.nodes[y][x] = &node{pos: Pos2{X: x, Y: y}}
			g.nodes[y][x].neighbors = make(map[Vec2]*node, 0)
			g.nodes[y][x].onShortestPath = false
		}
	}
}

func (from *node) accessible(to *node) bool {
	return to.height-from.height <= 1
}

func (from *node) addNeighbor(x, y, deltaX, deltaY int, g *grid) {
	to := g.nodes[y+deltaY][x+deltaX]
	if from.accessible(to) {
		from.neighbors[Vec2{X: deltaX, Y: deltaY}] = to
	}
}

func (g *grid) buildNeighborLists() {
	for y, line := range g.nodes {
		for x, node := range line {
			if x > 0 {
				node.addNeighbor(x, y, -1, 0, g)
			}
			if x < len(line)-1 {
				node.addNeighbor(x, y, 1, 0, g)
			}
			if y > 0 {
				node.addNeighbor(x, y, 0, -1, g)
			}
			if y < len(g.nodes)-1 {
				node.addNeighbor(x, y, 0, 1, g)
			}
		}
	}
}

func (g *grid) printDistances() {
	for _, line := range g.nodes {
		for _, node := range line {
			s := fmt.Sprintf("%4d", node.distance)
			fmt.Printf("%s", s)
		}
		fmt.Println()
	}
}

func (g *grid) print() {
	fmt.Printf("\nStart: (%d, %d)\n", g.start.pos.X, g.start.pos.Y)
	fmt.Printf("End:   (%d, %d)\n\n", g.end.pos.X, g.end.pos.Y)
	for _, line := range g.nodes {
		for _, node := range line {
			s := fmt.Sprintf("%3d", node.height)
			if node.height == 0 || node.height == 27 {
				s = CyanBackground(s)
			} else if node.onShortestPath {
				s = Cyan(s)
			} else if node.visited {
				s = Black(s)
			} else {
				s = "   "
			}
			fmt.Printf("%s", s)
		}
		fmt.Println()
	}
}
