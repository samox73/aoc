package utils

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strings"
)

func GetInput(day, year int) string {
	client := http.Client{}
	url := fmt.Sprintf("https://adventofcode.com/%d/day/%d/input", year, day)
	request, err := http.NewRequest("GET", url, nil)
	if err != nil {
		log.Fatal(err)
	}
	request.Header = http.Header{
		"Cookie": {"session=" + Read(".sessionid")},
	}
	response, err := client.Do(request)
	if err != nil {
		log.Fatal(err)
	}
	body, err := ioutil.ReadAll(response.Body)
	input := string(body)
	fmt.Println(input)
	return input
}

func Read(fileName string) string {
	contents, err := ioutil.ReadFile(fileName)
	if err != nil {
		log.Fatal(err)
	}
	return string(contents)
}

func SplitPair(text, c string) (string, string) {
	s := strings.Split(text, c)
	return s[0], s[1]
}

func SplitIntPair(text, c string) (int64, int64) {
	s1, s2 := SplitPair(text, c)
	return ToInt(s1), ToInt(s2)
}
