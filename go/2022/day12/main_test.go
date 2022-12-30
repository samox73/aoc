package main

import "testing"

func TestRuneConversionToHeight(t *testing.T) {
	runes := [...]rune{'a', 'b', 'c', 'z', 'S', 'E'}
	heights := [...]int{1, 2, 3, 26, 0, 27}
	for i, rune := range runes {
		height := heights[i]
		value := convertToHeight(rune)
		if value != height {
			t.Fatalf("convertToHeight(%c) = %d, but should be %d", rune, value, height)
		}
	}
}

func TestNodeAccessibleFromLower(t *testing.T) {
	a := &node{height: 0}
	b := &node{height: 1}
	if !a.accessible(b) {
		t.Fatalf("node{height: %d}.accessible(node{height: %d}) = false, but should be true", a.height, b.height)
	}
}

func TestNodeAccessibleFromHigher(t *testing.T) {
	a := &node{height: 21}
	b := &node{height: 1}
	if !a.accessible(b) {
		t.Fatalf("node{height: %d}.accessible(node{height: %d}) = false, but should be true", a.height, b.height)
	}
}

func TestNodeNotAccessibleFromLower(t *testing.T) {
	a := &node{height: 0}
	b := &node{height: 2}
	if a.accessible(b) {
		t.Fatalf("node{height: %d}.accessible(node{height: %d}) = false, but should be true", a.height, b.height)
	}
}
