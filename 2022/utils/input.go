package utils

import (
	"io/ioutil"
	"log"
)

func Read(fileName string) string {
	contents, err := ioutil.ReadFile(fileName)
	if err != nil {
		log.Fatal(err)
	}
	return string(contents)
}
