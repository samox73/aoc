package main

import (
	"fmt"
	"log"
	"strings"

	"github.com/samox73/aoc/go/2022/utils"
)

func main() {
	input := utils.GetInput(7, 2022)
	solutionA := PartA(input)
	solutionB := PartB(input)
	fmt.Printf("Day 07 solution A: %d\n", solutionA)
	fmt.Printf("Day 07 solution B: %d\n", solutionB)
}

func PartA(input string) int64 {
	root := ParseDir(input)
	fmt.Println(root.ToString(0))
	files := root.GetDirsWithMaxSize(100000)
	combinedSize := int64(0)
	for _, file := range files {
		combinedSize += file.GetSize()
	}
	return combinedSize
}

func PartB(input string) int64 {
	root := ParseDir(input)
	dirSize := root.GetSize()
	maxRootSize := int64(70000000 - 30000000)
	smallestSufficientSize := dirSize - maxRootSize
	fmt.Println(smallestSufficientSize)
	files := root.GetDirsWithMinSize(smallestSufficientSize)
	result := int64(9999999999999)
	for _, file := range files {
		size := file.GetSize()
		fmt.Println(size)
		if result > size && size > smallestSufficientSize {
			result = size
		}
	}
	return result
}

func ParseLs(lines []string) []File {
	var files []File
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}
		tokens := strings.Split(line, " ")
		size := int64(0)
		var dir *Dir
		fmt.Println(">> ", line)
		if tokens[0] == "dir" {
			dir = &Dir{}
		} else {
			dir = nil
			size = utils.ToInt(tokens[0])
		}
		file := File{name: tokens[1], size: size, dir: dir}
		fmt.Println(file.ToString(0))
		files = append(files, file)
	}
	return files
}

func ParseDir(input string) File {
	commands := strings.Split(input, "$ ")
	rootDir := Dir{}
	root := File{name: "/", size: 0, dir: &rootDir}
	cwd := &root
	for _, cmd := range commands {
		if len(cmd) == 0 {
			continue
		}
		fmt.Printf("> %s\n", cmd)
		lines := utils.GetLines(cmd)
		tokens := strings.Split(lines[0], " ")
		if tokens[0] == "ls" {
			files := ParseLs(lines[1:])
			cwd.AddFiles(files)
		} else if tokens[0] == "cd" {
			dirName := tokens[1]
			cwd = cwd.FindDir(dirName)
		} else {
			log.Fatal("Unknown cmd:\n" + cmd)
		}
	}
	return root
}
