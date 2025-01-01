// AoC 2024 Day 11: Plutionian Pebbles
package main

import (
	"aoc/utils"
	"fmt"
	"strconv"
)

type memoKey struct {
	pebble int
	steps  int
}

func count(pebble int, steps int, memo map[memoKey]int) int {
	key := memoKey{pebble, steps}
	if pebbleCount, ok := memo[key]; ok {
		return pebbleCount
	}

	if steps == 0 {
		return 1
	}

	pebbleStr := strconv.Itoa(pebble)

	pebbleCount := 0

	if pebble == 0 {
		pebbleCount = count(1, steps-1, memo)
	} else if len(pebbleStr)%2 == 0 {
		str := strconv.Itoa(pebble)
		left, _ := strconv.Atoi(str[:len(pebbleStr)/2])
		right, _ := strconv.Atoi(str[len(pebbleStr)/2:])
		pebbleCount = count(left, steps-1, memo) + count(right, steps-1, memo)
	} else {
		pebbleCount = count(pebble*2024, steps-1, memo)
	}
	memo[key] = pebbleCount

	return pebbleCount
}

func solvePart1(lines []string) int {
	defer utils.Timer("day11p1")()

	pebbles, _ := utils.SplitStringToIntSlice(lines[0], " ")

	total := 0
	memo := make(map[memoKey]int)
	for _, pebble := range pebbles {
		total += count(pebble, 25, memo)
	}

	return total
}

func solvePart2(lines []string) int {
	defer utils.Timer("day11p2")()

	pebbles, _ := utils.SplitStringToIntSlice(lines[0], " ")

	total := 0
	memo := make(map[memoKey]int)
	for _, pebble := range pebbles {
		total += count(pebble, 75, memo)
	}

	return total
}

func main() {
	fmt.Println("AoC 2024 - Day 11: Plutionian Pebbles")
	fmt.Println("=====================================")

	lines, err := utils.ReadLines("day11/day11_input.txt")

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	totalPart1 := solvePart1(lines)
	fmt.Println("Pebbles after 25 blinks (Part 1 Solution):", totalPart1)

	totalPart2 := solvePart2(lines)
	fmt.Println("Pebbles after 75 blinks (Part 2 Solution):", totalPart2)
}
