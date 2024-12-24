package main

import (
	"aoc/utils"
	"testing"
)

func TestParseLocation(t *testing.T) {
	var tests = []struct {
		input string
		want  location
	}{
		{"1 1", location{1, 1}},
		{"5 1", location{5, 1}},
		{"8 10", location{8, 10}},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			got, _ := parseLocation(tt.input)

			if got != tt.want {
				t.Errorf("Got %v, want %v", got, tt.want)
			}
		})
	}
}

func TestSampleInputPart1(t *testing.T) {
	lines, _ := utils.ReadLines("day01_sample.txt")
	got := solvePart1(lines)
	want := 11

	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}

func TestSampleInputPart2(t *testing.T) {
	lines, _ := utils.ReadLines("day01_sample.txt")
	got := solvePart2(lines)
	want := 31

	if got != want {
		t.Errorf("Got %v, want %v", got, want)
	}
}
