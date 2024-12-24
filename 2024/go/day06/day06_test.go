package main

import (
	"aoc/utils"
	"testing"
)

func TestSampleInputPart1(t *testing.T) {
	lines, _ := utils.ReadLines("day06_sample.txt")
	got := solvePart1(lines)
	want := 41

	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}

func TestSampleInputPart2(t *testing.T) {
	lines, _ := utils.ReadLines("day06_sample.txt")
	got := solvePart2(lines)
	want := 6

	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}
