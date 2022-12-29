package main

import "testing"

func TestRuneConversionToHeight(t *testing.T) {
	runes := [...]rune{'a', 'b', 'c', 'z'}
	heights := [...]int{0, 1, 2, 25}
	for i, rune := range runes {
		height := heights[i]
		value := convertToHeight(rune)
		if value != height {
			t.Fatalf("convertToHeight(%c) = %d, but should be %d", rune, value, height)
		}
	}
}
