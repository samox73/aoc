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
