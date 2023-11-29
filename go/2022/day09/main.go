package main

import (
	"fmt"
	"strconv"

	"github.com/cznic/mathutil"
	mapset "github.com/deckarep/golang-set/v2"

	"github.com/samox73/aoc/go/2022/utils"
)

type vec struct {
	x, y int
}

type knot struct {
	x, y int
	id   string
}

func (p *knot) add(v vec) {
	p.x += v.x
	p.y += v.y
}

func (p *knot) sub(that knot) {
	p.x -= that.x
	p.y -= that.y
}

func (this knot) equals(that knot) bool {
	return this.x == that.x && this.y == that.y
}

func absInt(i int) int {
	if i < 0 {
		return -i
	}
	return i
}

func distance(p1, p2 knot) int {
	p1.sub(p2)
	return mathutil.Max(absInt(p1.x), absInt(p1.y))
}

type snake []knot

func (s *snake) print(l, h knot) {
	for b := h.y; b >= l.y; b-- {
		for a := l.x; a <= h.x; a++ {
			id := "."
			for _, k := range *s {
				if k.equals(knot{x: a, y: b}) {
					id = k.id
					break
				}
			}
			fmt.Printf(id)
		}
		fmt.Printf("\n")
	}
	fmt.Printf("\n")
}

func (s *snake) move(v vec) {
	(*s)[0].add(v)
	for i := 1; i < len(*s); i++ {
		d := distance((*s)[i-1], (*s)[i])
		if d > 1 {
			diff := (*s)[i-1]
			diff.sub((*s)[i])
			(*s)[i].add(vec{
				x: mathutil.Clamp(diff.x, -1, 1),
				y: mathutil.Clamp(diff.y, -1, 1),
			})
		}
	}
}

func main() {
	input := utils.GetInput(9, 2022)
	solutionA := PartA(input)
	fmt.Println("here2")
	solutionB := PartB(input)
	fmt.Println("here3")
	fmt.Printf("Day 09 solution A: %d\n", solutionA)
	fmt.Printf("Day 09 solution B: %d\n", solutionB)
}

func getMove(input string) vec {
	if input == "U" {
		return vec{0, 1}
	} else if input == "R" {
		return vec{1, 0}
	} else if input == "D" {
		return vec{0, -1}
	} else if input == "L" {
		return vec{-1, 0}
	} else {
		return vec{0, 0}
	}
}

func parseMoves(input string) []vec {
	var moves []vec
	for _, line := range utils.GetLines(input) {
		m, c := utils.SplitPair(line, " ")
		move := getMove(m)
		count := utils.ToInt(c)
		for i := 0; i < count; i++ {
			moves = append(moves, move)
		}
	}
	return moves
}

func PartA(input string) int {
	fmt.Println(input)
	moves := parseMoves(input)
	s := snake{}
	s = append(s, knot{0, 0, "H"})
	s = append(s, knot{0, 0, "T"})
	visited := mapset.NewSet[knot]()
	for _, m := range moves {
		s.move(m)
		visited.Add(s[len(s)-1])
	}
	return visited.Cardinality()
}

func PartB(input string) int {
	fmt.Println(input)
	moves := parseMoves(input)
	s := snake{}
	s = append(s, knot{0, 0, "H"})
	for i := 1; i < 9; i++ {
		s = append(s, knot{0, 0, strconv.Itoa(i)})
	}
	s = append(s, knot{0, 0, "T"})
	visited := mapset.NewSet[knot]()
	for i, m := range moves {
		// only print the first 50 moves
		if i < 50 {
			a := 15
			s.print(knot{x: -a, y: -a}, knot{x: a, y: a})
			fmt.Printf("Move: (%d,%d)\n", m.x, m.y)
		}
		s.move(m)
		visited.Add(s[len(s)-1])
	}
	return visited.Cardinality()
}
