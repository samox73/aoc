package utils

import (
	"log"
	"strconv"
)

func ToInt(s string) int {
	marks, err := strconv.Atoi(s)
	if err != nil {
		log.Fatal(err)
	}
	return marks
}

func ToInts(arr []string) []int {
	var ints []int
	for _, s := range arr {
		i, err := strconv.Atoi(s)
		if err != nil {
			log.Fatal(err)
		}
		ints = append(ints, i)
	}
	return ints
}
