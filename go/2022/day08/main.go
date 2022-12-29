package main

import (
	"fmt"
	"strings"

	"github.com/samox73/aoc/go/2022/utils"
)

func main() {
	input := utils.GetInput(8, 2022)
	// input = "30373\n25512\n65332\n33549\n35390"
	solutionA := PartA(input)
	solutionB := PartB(input)
	fmt.Printf("Day 08 solution A: %d\n", solutionA)
	fmt.Printf("Day 08 solution B: %d\n", solutionB)
}

type Tree struct {
	height      int64
	visible     bool
	x, y        int
	scenicScore int64
}

type TreeField [][]Tree

func ParseField(input string) TreeField {
	lines := utils.GetLines(input)
	field := TreeField{}
	y := 0
	for _, line := range lines {
		fmt.Println(line)
		heights := strings.Split(line, "")
		var row []Tree
		x := 0
		for _, height := range heights {
			row = append(row, Tree{height: utils.ToInt(height), visible: false, x: x, y: y})
			x++
		}
		field = append(field, row)
		y++
	}
	return field
}

func setVisibilities(field TreeField) {
	lenY := len(field)
	for y := 0; y < lenY; y++ {
		lenX := len((field)[y])
		for x := 0; x < lenX; x++ {
			tree := &(field)[y][x]
			tree.setVisibility(field)
		}
	}
}

func (t *Tree) setVisibility(field TreeField) {
	visibleFromLeft := true
	for i := 0; i < t.x; i++ {
		if field[t.y][i].height >= t.height {
			visibleFromLeft = false
			break
		}
	}
	visibleFromRight := true
	for i := len(field[0]) - 1; i > t.x; i-- {
		if field[t.y][i].height >= t.height {
			visibleFromRight = false
			break
		}
	}
	visibleFromTop := true
	for i := 0; i < t.y; i++ {
		if field[i][t.x].height >= t.height {
			visibleFromTop = false
			break
		}
	}
	visibleFromBottom := true
	for i := len(field) - 1; i > t.y; i-- {
		if field[i][t.x].height >= t.height {
			visibleFromBottom = false
			break
		}
	}
	t.visible = visibleFromLeft || visibleFromRight || visibleFromTop || visibleFromBottom
	fmt.Printf("Tree at (%d,%d) is visible: %t\n", t.x, t.y, t.visible)
}

func PartA(input string) int64 {
	field := ParseField(input)
	setVisibilities(field)
	count := 0
	for _, row := range field {
		for _, tree := range row {
			char := "██  "
			if tree.visible {
				char = "░░  "
				count++
			}
			fmt.Printf("%s", char)
		}
		fmt.Println()
		fmt.Println()
	}
	return int64(count)
}

func setScenicScores(field TreeField) {
	lenY := len(field)
	for y := 0; y < lenY; y++ {
		lenX := len((field)[y])
		for x := 0; x < lenX; x++ {
			tree := &(field)[y][x]
			tree.setScenicScore(field)
		}
	}
}

func (t *Tree) setScenicScore(field TreeField) {
	lenY := len(field)
	lenX := len((field)[0])
	scoreRight := int64(0)
	for x := t.x + 1; x < lenX; x++ {
		scoreRight++
		if field[t.y][x].height >= t.height {
			break
		}
	}
	scoreLeft := int64(0)
	for x := t.x - 1; x >= 0; x-- {
		scoreLeft++
		if field[t.y][x].height >= t.height {
			break
		}
	}
	scoreDown := int64(0)
	for y := t.y + 1; y < lenY; y++ {
		scoreDown++
		if field[y][t.x].height >= t.height {
			break
		}
	}
	scoreUp := int64(0)
	for y := t.y - 1; y >= 0; y-- {
		scoreUp++
		if field[y][t.x].height >= t.height {
			break
		}
	}
	t.scenicScore = scoreDown * scoreUp * scoreLeft * scoreRight
}

func PartB(input string) int64 {
	field := ParseField(input)
	setScenicScores(field)
	scenicScore := int64(0)
	for _, row := range field {
		for _, tree := range row {
			if tree.scenicScore > scenicScore {
				scenicScore = tree.scenicScore
			}
		}
	}
	return scenicScore
}
