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

// var re = regexp.MustCompile(`Monkey (?P<src>\d):
//   Starting items: (?P<items>\d+(?:\s*,\s*\d+)+)
//   Operation: new = (?P<op>[old+*\/\d]+)
//   Test: divisible by (?P<test>\d+)
//     If true: throw to monkey (?P<dst1>\d+)
//     If false: throw to monkey (?P<dst2>\d+)`)

var re = regexp.MustCompile(`Monkey (?P<id>\d):
  Starting items: (?P<items>\d+(?:\s*,\s*\d+)+)
  Operation: new = (?P<operation>.+)
  Test: divisible by (?P<divisor>\d+)
    If true: throw to monkey (?P<destinationTrue>\d+)
    If false: throw to monkey (?P<destinationFalse>\d+)`)

type monkey struct {
	id               int
	items            []int
	expression       govaluate.EvaluableExpression
	divisor          int
	destinationTrue  int
	destinationFalse int
}

func printMonkeys(monkeys map[int]monkey) {
	keys := make([]int, 0, len(monkeys))
	for k := range monkeys {
		keys = append(keys, k)
	}
	sort.Ints(keys)
	for _, key := range keys {
		monkey := monkeys[key]
		fmt.Printf("\nMonkey #%d:", monkey.id)
		for _, item := range monkey.items {
			fmt.Printf(" %*d", 5, item)
		}
	}
	fmt.Printf("\n")
}

func PartA(input string) int {
	var monkeys map[int]monkey = make(map[int]monkey)
	groupNames := re.SubexpNames()
	for _, match := range re.FindAllStringSubmatch(input, -1) {
		// fmt.Printf("\nMatch #%d:\n", matchNum)
		m := monkey{}
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
			// fmt.Printf("#%d text: '%s', group: '%s'\n", matchNum, text, group)
		}
		fmt.Println(m)
		monkeys[m.id] = m
	}

	for m := range monkeys {
		monki := monkeys[m]
		fmt.Printf("\n")
		fmt.Printf("Evaluating Monkey #%d\n", monki.id)
		for i, item := range monkeys[m].items {
			parameters := make(map[string]interface{}, 1)
			parameters["old"] = item
			result, err := monki.expression.Evaluate(parameters)
			if err != nil {
				panic(err)
			}
			newVal := int(result.(float64) / 3)
			// fmt.Printf("\t- item %d -> %d\n", item, newVal)
			monkeys[m].items[i] = newVal
			var newMonkey monkey
			if (newVal % monki.divisor) == 0 {
				newMonkey = monkeys[monki.destinationTrue]
			} else {
				newMonkey = monkeys[monki.destinationFalse]
			}
			newMonkey.items = append(newMonkey.items, item)
		}
		monki.items = make([]int, 0)
		printMonkeys(monkeys)
	}

	return 0
}

func PartB(input string) int {
	return 0
}
