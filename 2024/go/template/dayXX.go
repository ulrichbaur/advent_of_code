// AoC 2024 Day XX
package main

import (
	"aoc/utils"
	"fmt"
)

func solvePart1(lines []string) int {
	defer utils.Timer("dayXXp1")()

	total := 0
	return total
}

func solvePart2(lines []string) int {
	defer utils.Timer("dayXXp2")()

	total := 0
	return total
}

func main() {
	fmt.Println("AoC 2024 - Day XX")
	fmt.Println("=================")

	lines, err := utils.ReadLines("day08/dayXX_sample.txt")

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}
	utils.PrintRows(lines)

	totalPart1 := solvePart1(lines)
	fmt.Println("... (Part 1 Solution):", totalPart1)

	totalPart2 := solvePart2(lines)
	fmt.Println("... (Part 2 Solution):", totalPart2)
}
