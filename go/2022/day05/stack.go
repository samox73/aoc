package main

import "fmt"

type Stack []string

func (stack *Stack) Put(crate string) {
	crateStack := &[]string{crate}
	*stack = append(*crateStack, *stack...)
}

func (stack *Stack) Pop() string {
	crate := (*stack)[0]
	*stack = (*stack)[1:]
	return crate
}

func (stack *Stack) Print() {
	for _, item := range *stack {
		fmt.Println(item)
	}
}

func PrintStacks(stacks []Stack) {
	maxSize := 0
	for _, stack := range stacks {
		size := len(stack)
		if maxSize < size {
			maxSize = size
		}
	}
	for i := 0; i < maxSize; i++ {
		for _, stack := range stacks {
			// stack.Print()
			if i < maxSize-len(stack) {
				fmt.Printf("    ")
			} else {
				fmt.Printf("%s ", stack[i-(maxSize-len(stack))])
			}
		}
		fmt.Println()
	}
}
