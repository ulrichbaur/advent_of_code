// AoC 2024 Day 10: Hoof It
package main

import (
	"aoc/utils"
	"fmt"
)

type position struct {
	x int
	y int
}

type direction position

var (
	directionUp    = direction{-1, 0}
	directionRight = direction{0, 1}
	directionDown  = direction{1, 0}
	directionLeft  = direction{0, -1}
)
var directions = []direction{
	directionUp,
	directionDown,
	directionLeft,
	directionRight,
}

func parseGrid(lines []string) [][]int {
	grid := make([][]int, 0)

	for _, line := range lines {
		row := make([]int, 0)
		for _, v := range line {
			numb := int(v - '0')
			row = append(row, numb)
		}
		grid = append(grid, row)
	}

	return grid
}

func isGraduallyIncreasing(curr int, prev int) bool {
	return curr-1 == prev
}

func isInBounds(grid [][]int, pos position) bool {
	return pos.x < len(grid) && pos.x >= 0 && pos.y < len(grid[0]) && pos.y >= 0
}

func searchPart1(grid [][]int, visited [][]int, prev int, pos position) int {
	if !isInBounds(grid, pos) {
		return 0
	}

	curr := grid[pos.x][pos.y]
	if !isGraduallyIncreasing(curr, prev) {
		return 0
	}

	if curr == 9 && prev == 8 {
		if visited[pos.x][pos.y] == 1 {
			return 0
		}
		visited[pos.x][pos.y] = 1
		return 1
	}

	score := 0
	for _, dir := range directions {
		newX := pos.x + dir.x
		newY := pos.y + dir.y
		score += searchPart1(grid, visited, curr, position{newX, newY})
	}

	return score
}

func searchPart2(grid [][]int, prev int, pos position) int {
	if !isInBounds(grid, pos) {
		return 0
	}

	curr := grid[pos.x][pos.y]
	if !isGraduallyIncreasing(curr, prev) {
		return 0
	}

	if curr == 9 && prev == 8 {
		return 1
	}

	score := 0
	for _, dir := range directions {
		newX := pos.x + dir.x
		newY := pos.y + dir.y
		score += searchPart2(grid, curr, position{newX, newY})
	}

	return score
}

func solvePart1(lines []string) int {
	defer utils.Timer("day10p1")()

	grid := parseGrid(lines)

	scores := make([]int, 0)

	for i := range len(grid) {
		for j := range len(grid[0]) {
			if grid[i][j] == 0 {
				visited := make([][]int, 0)
				for i := 0; i < len(grid); i++ {
					row := make([]int, len(grid[0]))
					visited = append(visited, row)
				}

				score := searchPart1(grid, visited, -1, position{i, j})
				scores = append(scores, score)
			}
		}
	}

	return utils.Sum(scores)
}

func solvePart2(lines []string) int {
	defer utils.Timer("day10p2")()

	grid := parseGrid(lines)

	scores := make([]int, 0)

	for i := range len(grid) {
		for j := range len(grid[0]) {
			if grid[i][j] == 0 {
				score := searchPart2(grid, -1, position{i, j})
				scores = append(scores, score)
			}
		}
	}

	return utils.Sum(scores)
}

func main() {
	fmt.Println("AoC 2024 - Day 10: Hoof It")
	fmt.Println("==========================")

	lines, err := utils.ReadLines("day10/day10_input.txt")

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	totalPart1 := solvePart1(lines)
	fmt.Println("Sum of all trailhead scores [distinct 9s reachable] (Part 1 Solution):", totalPart1)

	totalPart2 := solvePart2(lines)
	fmt.Println("Sum of all trailhead ratings [distinct trails] (Part 2 Solution):", totalPart2)
}
