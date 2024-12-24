package main

import (
	"aoc/utils"
	"testing"
)

func TestSampleInputPart1(t *testing.T) {
	lines, _ := utils.ReadLines("day03_sample1.txt")
	got := solvePart1(lines)
	want := 161

	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}

func TestSampleInputPart2(t *testing.T) {
	lines, _ := utils.ReadLines("day03_sample2.txt")
	got := solvePart2(lines)
	want := 48

	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}
