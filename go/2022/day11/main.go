package main

import (
	"fmt"
	"regexp"
	"sort"
	"strings"

	"github.com/Knetic/govaluate"

	"github.com/samox73/aoc/go/2022/utils"
)

func main() {
	input := utils.GetInput(11, 2022)
	solutionA := PartA(input)
	solutionB := PartB(input)
	fmt.Printf("Day 11 solution A: %d\n", solutionA)
	fmt.Printf("Day 11 solution B: %d\n", solutionB)
}

var re = regexp.MustCompile(`Monkey (?P<id>\d):
  Starting items: (?P<items>\d+(?:\s*,\s*\d+)*)
  Operation: new = (?P<operation>.+)
  Test: divisible by (?P<divisor>\d+)
    If true: throw to monkey (?P<destinationTrue>\d+)
    If false: throw to monkey (?P<destinationFalse>\d+)`)

type monkey struct {
	id               int
	items            []int
	inspectionCount  int
	expression       govaluate.EvaluableExpression
	divisor          int
	destinationTrue  int
	destinationFalse int
}

func getMonkeyBusiness(monkeys map[int]*monkey) int {
	keys := make([]int, 0, len(monkeys))
	for k := range monkeys {
		keys = append(keys, k)
	}
	sort.Slice(keys, func(i, j int) bool { return monkeys[keys[i]].inspectionCount > monkeys[keys[j]].inspectionCount })
	return monkeys[keys[0]].inspectionCount * monkeys[keys[1]].inspectionCount
}

func printMonkeys(monkeys map[int]*monkey) {
	keys := make([]int, 0, len(monkeys))
	for k := range monkeys {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	for _, key := range keys {
		monkey := monkeys[key]
		fmt.Printf("\nMonkey #%d (%3d):", monkey.id, monkey.inspectionCount)
		for _, item := range monkey.items {
			fmt.Printf(" %*d", 5, item)
		}
	}
	fmt.Printf("\n")
}

func buildMonkeys(input string) *map[int]*monkey {
	var monkeys map[int]*monkey = make(map[int]*monkey)
	groupNames := re.SubexpNames()
	for _, match := range re.FindAllStringSubmatch(input, -1) {
		m := &monkey{}
		m.inspectionCount = 0
		for groupIdx, text := range match {
			group := groupNames[groupIdx]
			switch group {
			case "id":
				m.id = utils.ToInt(text)
			case "items":
				m.items = utils.ToInts(strings.Split(text, ", "))
			case "operation":
				expression, err := govaluate.NewEvaluableExpression(text)
				if err != nil {
					panic(err)
				}
				m.expression = *expression
			case "divisor":
				m.divisor = utils.ToInt(text)
			case "destinationTrue":
				m.destinationTrue = utils.ToInt(text)
			case "destinationFalse":
				m.destinationFalse = utils.ToInt(text)
			}
		}
		monkeys[m.id] = m
	}
	return &monkeys
}

func runRound(monkeys *map[int]*monkey, f func(int) int, verbose bool) {
	if verbose {
		fmt.Printf("====== Starting new round ======")
	}
	keys := make([]int, 0, len(*monkeys))
	for k := range *monkeys {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	for _, m := range keys {
		monki := (*monkeys)[m]
		if verbose {
			fmt.Printf("\n")
			fmt.Printf("Evaluating Monkey #%d\n", monki.id)
		}
		for i, item := range monki.items {
			parameters := make(map[string]interface{}, 1)
			parameters["old"] = item
			result, err := monki.expression.Evaluate(parameters)
			if err != nil {
				panic(err)
			}
			newVal := f(int(result.(float64)))
			monki.items[i] = newVal
			dest := 0
			if (newVal % monki.divisor) == 0 {
				dest = monki.destinationTrue
			} else {
				dest = monki.destinationFalse
			}
			newMonkey := (*monkeys)[dest]
			newMonkey.items = append(newMonkey.items, newVal)
			monki.inspectionCount++
		}
		monki.items = make([]int, 0)
		(*monkeys)[m] = monki
		if verbose {
			printMonkeys(*monkeys)
		}
	}
}

func PartA(input string) int {
	monkeys := buildMonkeys(input)
	for i := 0; i < 20; i++ {
		runRound(monkeys, func(i int) int { return i / 3 }, true)
		printMonkeys(*monkeys)
	}
	return getMonkeyBusiness(*monkeys)
}

func getDivisorProduct(monkeys *map[int]*monkey) int {
	p := 1
	for _, m := range *monkeys {
		p *= m.divisor
	}
	return p
}

func PartB(input string) int {
	monkeys := buildMonkeys(input)
	p := getDivisorProduct(monkeys)
	for i := 0; i < 10000; i++ {
		runRound(monkeys, func(i int) int { return i % p }, false)
	}
	return getMonkeyBusiness(*monkeys)
}
