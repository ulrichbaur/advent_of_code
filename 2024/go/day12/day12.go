// AoC 2024 Day 12: Garden Groups
package main

import (
	"aoc/utils"
	"fmt"
)

type position struct {
	row int
	col int
}

func (p position) neighbours() []position {
	return []position{{p.row + 1, p.col}, {p.row - 1, p.col}, {p.row, p.col + 1}, {p.row, p.col - 1}}
}

type gardenRegion struct {
	plant string
	plots map[position]bool
}

func (r gardenRegion) area() int {
	return len(r.plots)
}

func (r gardenRegion) perimeter() int {
	visited := make(map[position]bool)
	perimeter := 0

	for pos := range r.plots {
		if _, ok := visited[pos]; ok {
			continue
		}
		visited[pos] = true

		for _, n := range pos.neighbours() {
			if _, ok2 := r.plots[n]; !ok2 {
				perimeter++
			}
		}
	}

	return perimeter
}

type offset struct {
	row int
	col int
}

func (r gardenRegion) sides() int {
	offsets := []offset{{-1, -1}, {-1, 1}, {1, 1}, {1, -1}}

	// counting corners of the region is equivalent to counting sides
	// as every side has 2 corners and every corner connects 2 sides
	corners := 0
	for pos := range r.plots {
		for _, o := range offsets {
			rowNeighbour := position{pos.row + o.row, pos.col}
			columnNeighbour := position{pos.row, pos.col + o.col}
			diagonalNeighbour := position{pos.row + o.row, pos.col + o.col}

			_, rowNeighborInPlots := r.plots[rowNeighbour]
			_, columnNeighbourInPlots := r.plots[columnNeighbour]
			_, diagonalNeighbourInPlots := r.plots[diagonalNeighbour]

			// exterior corner (classical corners, for example, the corners of a square)
			if !rowNeighborInPlots && !columnNeighbourInPlots {
				corners++
			}
			// interior corner (for example, corners in the center of plus [+] symbol)
			if rowNeighborInPlots && columnNeighbourInPlots && !diagonalNeighbourInPlots {
				corners++
			}
		}
	}
	return corners
}

func floodFill(garden map[position]string, startPos position, plant string, visited map[position]bool) map[position]bool {
	set := make(map[position]bool)

	if _, ok := visited[startPos]; ok {
		return set
	}

	if p, ok := garden[startPos]; !ok || p != plant {
		return set
	}

	set[startPos] = true
	visited[startPos] = true

	for _, n := range startPos.neighbours() {
		for pos := range floodFill(garden, n, plant, visited) {
			set[pos] = true
		}
	}

	return set
}

func solvePart1(lines []string) int {
	defer utils.Timer("day12p1")()

	garden := make(map[position]string)
	for rowIndex, line := range lines {
		for colIndex, ch := range line {
			garden[position{rowIndex, colIndex}] = string(ch)
		}
	}

	visited := make(map[position]bool)
	regions := make([]gardenRegion, 0)

	for pos, plant := range garden {
		region := floodFill(garden, pos, plant, visited)
		if len(region) > 0 {
			regions = append(regions, gardenRegion{plant, region})
		}
	}

	price := 0
	for _, r := range regions {
		price += r.area() * r.perimeter()
	}
	return price
}

func solvePart2(lines []string) int {
	defer utils.Timer("day12p2")()

	garden := make(map[position]string)
	for rowIndex, line := range lines {
		for colIndex, ch := range line {
			garden[position{rowIndex, colIndex}] = string(ch)
		}
	}

	visited := make(map[position]bool)
	regions := make([]gardenRegion, 0)

	for pos, plant := range garden {
		region := floodFill(garden, pos, plant, visited)
		if len(region) > 0 {
			regions = append(regions, gardenRegion{plant, region})
		}
	}

	price := 0
	for _, r := range regions {
		price += r.area() * r.sides()
	}
	return price
}

func main() {
	fmt.Println("AoC 2024 - Day 12: Garden Groups")
	fmt.Println("================================")

	lines, err := utils.ReadLines("day12/day12_input.txt")

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	totalPart1 := solvePart1(lines)
	fmt.Println("Total price of fencing all regions [perimeter-based] (Part 1 Solution):", totalPart1)

	totalPart2 := solvePart2(lines)
	fmt.Println("Total price of fencing all regions [sides-based] (Part 2 Solution):", totalPart2)
}
