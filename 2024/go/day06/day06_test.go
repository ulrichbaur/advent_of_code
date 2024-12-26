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
		{"day 06 sample", "day06_sample.txt", 41},
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
		{"day 06 sample", "day06_sample.txt", 6},
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
