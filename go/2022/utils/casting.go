package utils

import (
	"log"
	"strconv"
)

func ToInt(s string) int64 {
	marks, err := strconv.ParseInt(s, 10, 0)
	if err != nil {
		log.Fatal(err)
	}
	return marks
}

func ToInts(arr []string) []int64 {
	var ints []int64
	for _, s := range arr {
		i, err := strconv.ParseInt(s, 10, 0)
		if err != nil {
			log.Fatal(err)
		}
		ints = append(ints, i)
	}
	return ints
}
