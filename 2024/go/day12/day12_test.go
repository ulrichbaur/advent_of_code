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
		{"day 12 sample 1", "day12_sample1.txt", 140},
		{"day 12 sample 2", "day12_sample2.txt", 772},
		{"day 12 sample 3", "day12_sample3.txt", 1930},
		{"day 12 sample 4", "day12_sample4.txt", 692},
		{"day 12 sample 5", "day12_sample5.txt", 1184},
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
		{"day 12 sample 1", "day12_sample1.txt", 80},
		{"day 12 sample 2", "day12_sample2.txt", 436},
		{"day 12 sample 3", "day12_sample3.txt", 1206},
		{"day 12 sample 4", "day12_sample4.txt", 236},
		{"day 12 sample 5", "day12_sample5.txt", 368},
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
