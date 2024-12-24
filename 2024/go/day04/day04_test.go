package main

import (
	"aoc/utils"
	"testing"
)

func TestSampleInputPart1(t *testing.T) {
	lines, _ := utils.ReadLines("day04_sample.txt")
	got := solvePart1(lines)
	want := 18

	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}

func TestSampleInputPart2(t *testing.T) {
	lines, _ := utils.ReadLines("day04_sample.txt")
	got := solvePart2(lines)
	want := 9

	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}
