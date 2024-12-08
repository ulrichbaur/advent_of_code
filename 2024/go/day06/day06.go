// AoC 2024 Day 06
package main

import (
	"aoc/utils"
	"fmt"
	"strings"
)

// Point is defined by the row and column inside a grid.
type Point struct {
	x int
	y int
}

func findStartingPosition(lines []string) Point {
	for i, line := range lines {
		pos := strings.Index(line, "^")
		if pos != -1 {
			return Point{i, pos}
		}
	}
	return Point{-1, -1}
}

// Direction the guard is moving in.
type Direction Point

// All the possible directions to move inside the grid.
var (
	DirectionUp    = Direction{-1, 0}
	DirectionRight = Direction{0, 1}
	DirectionDown  = Direction{1, 0}
	DirectionLeft  = Direction{0, -1}
)

// Guard contains all information about the guard at any given point in time.
type Guard struct {
	position  Point
	direction Direction
}

func isObstacle(position Point, field [][]rune) bool {
	return field[position.x][position.y] == '#' || field[position.x][position.y] == 'O'
}

func changeDirection(direction Direction) Direction {
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

func moveGuard(guard Guard) Guard {
	next := guard
	next.position.x = guard.position.x + guard.direction.x
	next.position.y = guard.position.y + guard.direction.y
	return next
}

func move(guard Guard, field [][]rune, visited map[Point][]Guard) (bool, [][]rune, map[Point][]Guard) {
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

func visitPositions(grid [][]rune, start Point) []Point {
	guard := Guard{start, DirectionUp}
	visited := make(map[Point][]Guard)
	_, _, visited = move(guard, grid, visited)

	visitedList := make([]Point, 0, len(visited))

	for pos := range visited {
		visitedList = append(visitedList, pos)
	}

	return visitedList
}

func calculateObstructionSpots(grid [][]rune, path []Point, start Point) []Point {
	obstructions := make([]Point, 0)
	guard := Guard{start, DirectionUp}

	for _, point := range path {
		if point.x == start.x && point.y == start.y {
			continue
		}

		// place obstruction at point and check if it's a loop
		grid[point.x][point.y] = '#'
		visited := make(map[Point][]Guard)
		loop, _, _ := move(guard, grid, visited)
		grid[point.x][point.y] = '.'
		if loop {
			obstructions = append(obstructions, point)
		}
	}

	return obstructions
}

func solvePart1(lines []string) int {
	start := findStartingPosition(lines)
	grid := splitStringsIntoChars(lines)
	visited := visitPositions(grid, start)
	return len(visited)
}

func solvePart2(lines []string) int {
	start := findStartingPosition(lines)

	grid := splitStringsIntoChars(lines)
	visited := visitPositions(grid, start)
	obstructions := calculateObstructionSpots(grid, visited, start)

	return len(obstructions)
}

func main() {
	fmt.Println("AoC 2024 - Day 6")
	fmt.Println("==================")

	lines, err := utils.ReadLines("day06/day06_input.txt")

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}
	utils.PrintRows(lines)

	totalPart1 := solvePart1(lines)
	totalPart2 := solvePart2(lines)

	fmt.Println("Distinct positions (Part 1 Solution):", totalPart1)
	fmt.Println("Loop creating obstructions (Part 2 Solution):", totalPart2)
}
