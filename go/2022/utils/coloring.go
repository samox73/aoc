package utils

import (
	"fmt"
)

const colorNone = "\033[0m"

type colorable interface {
	~int | ~string
}

func ColorFromRGB(r, g, b int) string {
	return fmt.Sprintf("\033[38;2;%d;%d;%dm", r, g, b)
}

func BackgroundFromRGB(r, g, b int) string {
	return fmt.Sprintf("\033[48;2;%d;%d;%dm", r, g, b)
}

func Color[K colorable](v K, r, g, b int) string {
	c := fmt.Sprintf("%v", v)
	return ColorFromRGB(r, g, b) + c + colorNone
}

func Black[K colorable](v K) string {
	c := fmt.Sprintf("%v", v)
	return ColorFromRGB(0, 0, 0) + c + colorNone
}

func Cyan[K colorable](v K) string {
	c := fmt.Sprintf("%v", v)
	return ColorFromRGB(0, 255, 255) + c + colorNone
}

func CyanBackground[K colorable](v K) string {
	c := fmt.Sprintf("%v", v)
	return BackgroundFromRGB(0, 255, 255) + c + colorNone
}
