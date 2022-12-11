package main

import (
	"fmt"
	"log"
	"strings"
)

type File struct {
	name   string
	size   int64
	dir    *Dir
	parent *File
}

func (f *File) ChildrenCount() int {
	if !f.IsDir() {
		return 0
	}
	return len(*(f.dir))
}

func (f *File) IsDir() bool {
	return f.dir != nil
}

func (f *File) Add(other File) {
	if !f.IsDir() {
		log.Fatal("Cannot append a file to a file")
	}
	d := f.dir
	other.parent = f
	*d = append(*d, other)
}

func (f *File) findRoot() *File {
	if f.parent != nil {
		return f.parent.findRoot()
	} else {
		return f
	}
}

func (f *File) FindDir(name string) *File {
	if !f.IsDir() {
		log.Fatal("File " + name + " is not a directory")
	}
	if name == ".." {
		return f.parent
	} else if name == "/" {
		return f.findRoot()
	}
	d := f.dir
	for _, subDir := range *d {
		if subDir.name == name {
			return &subDir
		}
	}
	log.Fatal(fmt.Sprintf("Could not find subdir with name '%s'", name))
	return nil
}

func (f *File) AddFiles(files []File) {
	for _, file := range files {
		f.Add(file)
	}
}

func (f *File) GetDirsWithMaxSize(maxSize int64) []File {
	var files []File
	if f.IsDir() {
		for _, file := range *f.dir {
			if file.IsDir() && file.GetSize() <= maxSize {
				files = append(files, file)
			}
			files = append(files, file.GetDirsWithMaxSize(maxSize)...)
		}
	}
	return files
}

func (f *File) GetDirsWithMinSize(minSize int64) []File {
	var files []File
	if f.IsDir() {
		for _, file := range *f.dir {
			if file.IsDir() && file.GetSize() >= minSize {
				files = append(files, file)
			}
			files = append(files, file.GetDirsWithMinSize(minSize)...)
		}
	}
	return files
}

func (f *File) ToString(tab int) string {
	s := ""
	if f.IsDir() {
		d := f.dir
		s += fmt.Sprintf("%s- %s (dir)\n", strings.Repeat("  ", tab), f.name)
		for _, file := range *d {
			s += file.ToString(tab + 1)
		}
	} else {
		s += fmt.Sprintf("%s- %s (file, size=%d)\n", strings.Repeat("  ", tab), f.name, f.size)
	}
	return s
}

type Dir []File

func (f *File) GetSize() int64 {
	size := int64(0)
	if f.IsDir() {
		d := f.dir
		for _, file := range *d {
			size += file.GetSize()
		}
	} else {
		size += f.size
	}
	return size
}
