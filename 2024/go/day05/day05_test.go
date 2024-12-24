package main

import (
	"aoc/utils"
	"testing"
)

func TestSampleInputPart1(t *testing.T) {
	lines, _ := utils.ReadLines("day05_sample.txt")
	got := solvePart1(lines)
	want := 143

	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}

func TestSampleInputPart2(t *testing.T) {
	lines, _ := utils.ReadLines("day05_sample.txt")
	got := solvePart2(lines)
	want := 123

	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}
