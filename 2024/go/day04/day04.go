// AoC 2024 Day 04
package main

import (
	"aoc/utils"
	"fmt"
	"strings"
)

func transposeRows(rows []string) []string {
	columnLen := len(rows[0])
	columns := make([]string, columnLen)

	for colIndex := 0; colIndex < columnLen; colIndex++ {
		str := ""
		for _, row := range rows {
			str += string(row[colIndex])
		}
		columns[colIndex] = str
	}
	return columns
}

// createDiagonalsLeftToRight creates a diagonal from the bottom left to the top right
//
//	    x
//	  x
//	x
func createDiagonalsLeftToRight(rows []string) []string {
	rowLen := len(rows)
	columnLen := len(rows[0])

	diagonals := make([]string, 0)

	for k := 0; k < rowLen; k++ {
		diagonal := ""
		for rowIndex, colIndex := k, 0; rowIndex >= 0 && colIndex < columnLen; {
			diagonal += string(rows[rowIndex][colIndex])
			rowIndex--
			colIndex++
		}
		diagonals = append(diagonals, diagonal)
	}
	for k := 1; k < columnLen; k++ {
		diagonal := ""
		for rowIndex, colIndex := rowLen-1, k; rowIndex >= 0 && colIndex < columnLen; {
			diagonal += string(rows[rowIndex][colIndex])
			rowIndex--
			colIndex++
		}
		diagonals = append(diagonals, diagonal)
	}
	return diagonals
}

// createDiagonalsRightToLeft creates a diagonal from the top left to the bottom right
//
//	x
//	  x
//	    x
func createDiagonalsRightToLeft(rows []string) []string {
	rowLen := len(rows)
	columnLen := len(rows[0])

	diagonals := make([]string, 0)

	for k := rowLen - 1; k >= 0; k-- {
		diagonal := ""
		for rowIndex, colIndex := k, 0; rowIndex < rowLen && colIndex < columnLen; {
			diagonal += string(rows[rowIndex][colIndex])
			rowIndex++
			colIndex++
		}
		diagonals = append(diagonals, diagonal)
	}
	for k := 1; k < columnLen; k++ {
		diagonal := ""
		for rowIndex, colIndex := 0, k; rowIndex < rowLen && colIndex < columnLen; {
			diagonal += string(rows[rowIndex][colIndex])
			rowIndex++
			colIndex++
		}
		diagonals = append(diagonals, diagonal)
	}
	return diagonals
}

func solvePart1(rows []string) int {
	columns := transposeRows(rows)
	diagonals := createDiagonalsLeftToRight(rows)
	diagonals2 := createDiagonalsRightToLeft(rows)

	total := 0
	for _, row := range rows {
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

func solvePart2(rows []string) int {
	total := 0
	// 1) search for "A"; skip both first and last row as well as first and last column
	// 2) then check
	//      - top left is either S or M, and the other one on the bottom right
	//      - bottom left is either S or M, and the other one on the top right

	rowCount := len(rows)
	colCount := len(rows[0])
	for rowIndex := 1; rowIndex < rowCount-1; rowIndex++ {
		colPos := 1
		for colPos < colCount-1 {
			pos := strings.Index(rows[rowIndex][colPos:colCount-1], "A")
			if pos == -1 {
				break
			}
			colIndex := colPos + pos
			colPos = colIndex + 1

			rowAbove := rows[rowIndex-1]
			rowBelow := rows[rowIndex+1]
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
	fmt.Println("AoC 2024 - Day 4")
	fmt.Println("==================")

	lines, err := utils.ReadLines("day04/day04_input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	totalPart1 := solvePart1(lines)
	totalPart2 := solvePart2(lines)

	fmt.Println("Total XMAS count (Part 1 Solution):", totalPart1)
	fmt.Println("Total X-MAS count (Part 2 Solution):", totalPart2)
}
