// AoC 2024 Day 04: Ceres Search
package main

import (
	"aoc/utils"
	"fmt"
	"strings"
)

func solvePart1(lines []string) int {
	defer utils.Timer("day04p1")()

	columns := utils.TransposeRows(lines)
	diagonals := utils.CreateDiagonalsLeftToRight(lines)
	diagonals2 := utils.CreateDiagonalsRightToLeft(lines)

	total := 0
	for _, row := range lines {
		total += strings.Count(row, "XMAS") + strings.Count(row, "SAMX")
	}
	for _, column := range columns {
		total += strings.Count(column, "XMAS") + strings.Count(column, "SAMX")
	}
	for _, diagonal := range diagonals {
		total += strings.Count(diagonal, "XMAS") + strings.Count(diagonal, "SAMX")
	}
	for _, diagonal := range diagonals2 {
		total += strings.Count(diagonal, "XMAS") + strings.Count(diagonal, "SAMX")
	}
	return total
}

func solvePart2(lines []string) int {
	defer utils.Timer("day04p2")()

	total := 0
	// 1) search for "A"; skip both first and last row as well as first and last column
	// 2) then check
	//      - top left is either S or M, and the other one on the bottom right
	//      - bottom left is either S or M, and the other one on the top right

	rowCount := len(lines)
	colCount := len(lines[0])
	for rowIndex := 1; rowIndex < rowCount-1; rowIndex++ {
		colPos := 1
		for colPos < colCount-1 {
			pos := strings.Index(lines[rowIndex][colPos:colCount-1], "A")
			if pos == -1 {
				break
			}
			colIndex := colPos + pos
			colPos = colIndex + 1

			rowAbove := lines[rowIndex-1]
			rowBelow := lines[rowIndex+1]
			// check if top left is S or M
			topLeft := string(rowAbove[colIndex-1])
			bottomRight := string(rowBelow[colIndex+1])
			if topLeft == "M" || topLeft == "S" {
				if bottomRight == "M" || bottomRight == "S" {
					if bottomRight != topLeft {
						topRight := string(rowAbove[colIndex+1])
						bottomLeft := string(rowBelow[colIndex-1])
						if topRight == "M" || topRight == "S" {
							if bottomLeft == "M" || bottomLeft == "S" {
								if topRight != bottomLeft {
									total++
								}
							}
						}
					}
				}
			}
		}
	}
	return total
}

func main() {
	fmt.Println("AoC 2024 - Day 04: Ceres Search")
	fmt.Println("===============================")

	lines, err := utils.ReadLines("day04/day04_input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	totalPart1 := solvePart1(lines)
	fmt.Println("Total XMAS count (Part 1 Solution):", totalPart1)

	totalPart2 := solvePart2(lines)
	fmt.Println("Total X-MAS count (Part 2 Solution):", totalPart2)
}
