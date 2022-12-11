package main

import (
	"fmt"
	"strings"
)

type MovingQueue struct {
	markers []string
	size    int
	maxSize int
}

func (d *MovingQueue) add(marker string) {
	(*d).markers = append((*d).markers[1:], marker)
	if d.size < d.maxSize {
		(*d).size++
	}
}

func (d *MovingQueue) unique() bool {
	for _, marker := range d.markers {
		if strings.Count(strings.Join(d.markers, ""), marker) != 1 {
			return false
		}
	}
	return true
}

func (d *MovingQueue) print() {
	for _, marker := range (*d).markers {
		if len(marker) == 0 {
			fmt.Printf("  ")
		} else {
			fmt.Printf("%s ", marker)
		}
	}
	fmt.Println()
}
