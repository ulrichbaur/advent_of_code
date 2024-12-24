// AoC 2024 Day 01: Historian Hysteria
package main

import (
	"aoc/utils"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type locationPairs struct {
	left  []int
	right []int
}

type location struct {
	left  int
	right int
}

func parseLocation(line string) (location, error) {
	left := 0
	right := 0

	parts := strings.Fields(line)
	if len(parts) != 2 {
		return location{left, right}, fmt.Errorf("Invalid input, expected 2 parts in line: %q", line)
	}

	for i, part := range parts {
		num, err := strconv.Atoi(part)
		if err != nil {
			return location{left, right}, fmt.Errorf("Invalid number %q in %q: %v", part, line, err)
		}

		if i == 0 {
			left = num
		} else {
			right = num
		}
	}

	return location{left, right}, nil
}

func parseLocationLists(lines []string) (*locationPairs, error) {
	var locations locationPairs
	for _, line := range lines {
		loc, err := parseLocation(line)
		if err != nil {
			return nil, err
		}
		locations.left = append(locations.left, loc.left)
		locations.right = append(locations.right, loc.right)
	}

	return &locations, nil
}

func calculateDistances(locations locationPairs) []int {
	var distances []int
	for i := range len(locations.left) {
		distances = append(distances, utils.Abs(locations.left[i]-locations.right[i]))
	}
	return distances
}

func calculateSimilarityScores(locations locationPairs) []int {
	countMap := make(map[int]int)
	for _, num := range locations.right {
		countMap[num]++
	}

	var similarityScores []int
	for _, num := range locations.left {
		similarityScores = append(similarityScores, num*countMap[num])
	}
	return similarityScores
}

func solvePart1(lines []string) int {
	defer utils.Timer("day01p1")()

	locations, _ := parseLocationLists(lines)

	sort.Ints(locations.left)
	sort.Ints(locations.right)

	distances := calculateDistances(*locations)
	return utils.Sum(distances)
}

func solvePart2(lines []string) int {
	defer utils.Timer("day01p2")()

	locations, _ := parseLocationLists(lines)

	sort.Ints(locations.left)
	sort.Ints(locations.right)

	distances := calculateSimilarityScores(*locations)
	return utils.Sum(distances)
}

func main() {
	fmt.Println("AoC 2024 - Day 01: Historian Hysteria")
	fmt.Println("=====================================")

	lines, err := utils.ReadLines("day01/day01_input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	totalDistance := solvePart1(lines)
	fmt.Println("Total distance (Part 1 Solution):", totalDistance)

	totalSimilarityScore := solvePart2(lines)
	fmt.Println("Total Similarity Score (Part 2 Solution):", totalSimilarityScore)
}
