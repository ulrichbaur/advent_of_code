package main

import (
	"aoc/utils"
	"testing"
)

func TestPart1(t *testing.T) {
	var tests = []struct {
		name      string
		inputFile string
		want      int
	}{
		{"day 01 sample", "day01_sample.txt", 11},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lines, _ := utils.ReadLines(tt.inputFile)

			got := solvePart1(lines)

			if got != tt.want {
				t.Errorf("Got %v, want %v", got, tt.want)
			}
		})
	}
}

func TestPart2(t *testing.T) {
	var tests = []struct {
		name      string
		inputFile string
		want      int
	}{
		{"day 01 sample", "day01_sample.txt", 31},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lines, _ := utils.ReadLines(tt.inputFile)

			got := solvePart2(lines)

			if got != tt.want {
				t.Errorf("Got %v, want %v", got, tt.want)
			}
		})
	}
}

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
