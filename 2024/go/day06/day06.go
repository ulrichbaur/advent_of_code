// AoC 2024 Day 06: Guard Gallivant
package main

import (
	"aoc/utils"
	"fmt"
	"strings"
)

// Point is defined by the row and column inside a grid.
type point struct {
	x int
	y int
}

func findStartingPosition(lines []string) point {
	for i, line := range lines {
		pos := strings.Index(line, "^")
		if pos != -1 {
			return point{i, pos}
		}
	}
	return point{-1, -1}
}

// Direction the guard is moving in.
type direction point

// All the possible directions to move inside the grid.
var (
	DirectionUp    = direction{-1, 0}
	DirectionRight = direction{0, 1}
	DirectionDown  = direction{1, 0}
	DirectionLeft  = direction{0, -1}
)

// Guard contains all information about the guard at any given point in time.
type guard struct {
	position  point
	direction direction
}

func isObstacle(position point, field [][]rune) bool {
	return field[position.x][position.y] == '#' || field[position.x][position.y] == 'O'
}

func changeDirection(direction direction) direction {
	switch direction {
	case DirectionUp:
		return DirectionRight
	case DirectionDown:
		return DirectionLeft
	case DirectionLeft:
		return DirectionUp
	default:
		return DirectionDown
	}
}

func moveGuard(guard guard) guard {
	next := guard
	next.position.x = guard.position.x + guard.direction.x
	next.position.y = guard.position.y + guard.direction.y
	return next
}

func move(guard guard, field [][]rune, visited map[point][]guard) (bool, [][]rune, map[point][]guard) {
	rowLength := len(field[0])
	rowCount := len(field)

	if (guard.position.x == 0 && guard.direction == DirectionUp) ||
		(guard.position.x == rowCount-1 && guard.direction == DirectionDown) ||
		(guard.position.y == 0 && guard.direction == DirectionLeft) ||
		(guard.position.y == rowLength-1 && guard.direction == DirectionRight) {
		visited[guard.position] = append(visited[guard.position], guard)
		return false, field, visited
	}

	next := moveGuard(guard)

	// is there an obstacle?
	if isObstacle(next.position, field) {
		// fmt.Printf("Obstacle at %v\n", next.position)
		guard.direction = changeDirection(guard.direction)
		return move(guard, field, visited)
	}

	// was I here before?
	previousVisit, ok := visited[guard.position]

	if ok {
		// fmt.Println("previous Visits:", previousVisit)
		for _, visit := range previousVisit {
			if visit.direction == guard.direction {
				return true, field, visited
			}
		}
	}
	visited[guard.position] = append(visited[guard.position], guard)
	return move(next, field, visited)
}

func splitStringsIntoChars(lines []string) [][]rune {
	chars := make([][]rune, 0, len(lines)*len(lines[0]))
	for _, line := range lines {
		strRunes := []rune(line)
		chars = append(chars, strRunes)
	}
	return chars
}

func visitPositions(grid [][]rune, start point) []point {
	g := guard{start, DirectionUp}
	visited := make(map[point][]guard)
	_, _, visited = move(g, grid, visited)

	visitedList := make([]point, 0, len(visited))

	for pos := range visited {
		visitedList = append(visitedList, pos)
	}

	return visitedList
}

func calculateObstructionSpots(grid [][]rune, path []point, start point) []point {
	obstructions := make([]point, 0)
	g := guard{start, DirectionUp}

	for _, pos := range path {
		if pos.x == start.x && pos.y == start.y {
			continue
		}

		// place obstruction at point and check if it's a loop
		grid[pos.x][pos.y] = '#'
		visited := make(map[point][]guard)
		loop, _, _ := move(g, grid, visited)
		grid[pos.x][pos.y] = '.'
		if loop {
			obstructions = append(obstructions, pos)
		}
	}

	return obstructions
}

func solvePart1(lines []string) int {
	defer utils.Timer("day06p1")()
	start := findStartingPosition(lines)
	grid := splitStringsIntoChars(lines)
	visited := visitPositions(grid, start)
	return len(visited)
}

func solvePart2(lines []string) int {
	defer utils.Timer("day06p2")()
	start := findStartingPosition(lines)
	grid := splitStringsIntoChars(lines)
	visited := visitPositions(grid, start)
	obstructions := calculateObstructionSpots(grid, visited, start)

	return len(obstructions)
}

func main() {
	fmt.Println("AoC 2024 - Day 06: Guard Gallivant")
	fmt.Println("==================================")

	lines, err := utils.ReadLines("day06/day06_input.txt")

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}
	// utils.PrintRows(lines)

	totalPart1 := solvePart1(lines)
	fmt.Println("Distinct positions (Part 1 Solution):", totalPart1)

	totalPart2 := solvePart2(lines)
	fmt.Println("Loop creating obstructions (Part 2 Solution):", totalPart2)
}
