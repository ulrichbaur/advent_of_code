// AoC 2024 Day 08
package main

import (
	"aoc/utils"
	"fmt"
)

type node2d struct {
	x int
	y int
}

type antenna node2d

type antinode node2d

func isValidAntinode(node antinode, maxRows, maxCols int) bool {
	if node.x < 0 || node.y < 0 {
		return false
	}

	if node.x >= maxRows || node.y >= maxCols {
		return false
	}

	return true
}

func findExtendedAntinodes(antenna1, antenna2 antenna, maxRows, maxCols int) []antinode {
	antinodes := make([]antinode, 0)
	a1 := antenna1
	a2 := antenna2
	for i := 0; ; i++ {
		antinode := antinode{a1.x - i*(a2.x-a1.x), a1.y - i*(a2.y-a1.y)}
		if isValidAntinode(antinode, maxRows, maxCols) {
			antinodes = append(antinodes, antinode)
		} else {
			break
		}
	}
	// swap direction
	a1 = antenna2
	a2 = antenna1
	for i := 0; ; i++ {
		antinode := antinode{a1.x - i*(a2.x-a1.x), a1.y - i*(a2.y-a1.y)}
		if isValidAntinode(antinode, maxRows, maxCols) {
			antinodes = append(antinodes, antinode)
		} else {
			break
		}
	}

	return antinodes
}

func parseAntennas(lines []string) map[byte][]antenna {
	antennas := make(map[byte][]antenna)

	for i := range len(lines) {
		for j := range len(lines[0]) {
			freq := lines[i][j]
			if freq != '.' {
				freqAntennas, exists := antennas[freq]
				if exists {
					antennas[freq] = append(freqAntennas, antenna{i, j})
				} else {
					antennas[freq] = []antenna{{i, j}}
				}
			}
		}
	}
	return antennas
}

func solvePart1(lines []string) int {
	defer utils.Timer("day08p1")()

	antennas := parseAntennas(lines)

	antinodes := make(map[antinode]bool)

	for _, freqAntennas := range antennas {
		for i := range len(freqAntennas) {
			antenna1 := freqAntennas[i]
			for j := i + 1; j < len(freqAntennas); j++ {
				antenna2 := freqAntennas[j]

				antinode1 := antinode{2*antenna1.x - antenna2.x, 2*antenna1.y - antenna2.y}
				if isValidAntinode(antinode1, len(lines), len(lines[0])) {
					antinodes[antinode1] = true
				}
				antinode2 := antinode{2*antenna2.x - antenna1.x, 2*antenna2.y - antenna1.y}
				if isValidAntinode(antinode2, len(lines), len(lines[0])) {
					antinodes[antinode2] = true
				}
			}
		}
	}

	return len(antinodes)
}

func solvePart2(lines []string) int {
	defer utils.Timer("day08p2")()

	antennas := parseAntennas(lines)

	antinodes := make(map[antinode]bool)
	for _, freqAntennas := range antennas {
		for i := range len(freqAntennas) {
			antenna1 := freqAntennas[i]
			for j := i + 1; j < len(freqAntennas); j++ {
				antenna2 := freqAntennas[j]

				antinodeBatch := findExtendedAntinodes(antenna1, antenna2, len(lines), len(lines[0]))

				for _, antinode := range antinodeBatch {
					antinodes[antinode] = true
				}
			}
		}
	}
	return len(antinodes)
}

func main() {
	fmt.Println("AoC 2024 - Day 8")
	fmt.Println("==================")

	lines, err := utils.ReadLines("day08/day08_input.txt")

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}
	// utils.PrintRows(lines)

	totalPart1 := solvePart1(lines)
	fmt.Println("Unique positions of antinodes (Part 1 Solution):", totalPart1)

	totalPart2 := solvePart2(lines)
	fmt.Println("Unique positions of antinodes (Part 2 Solution):", totalPart2)
}
