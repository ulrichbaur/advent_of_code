// AoC 2024 Day 07
package main

import (
	"aoc/utils"
	"fmt"
	"strconv"
	"strings"
)

type equation struct {
	testValue int
	operands  []int
}

func concatInts(n1, n2 int) int {
	s := fmt.Sprintf("%d%d", n1, n2)
	num, _ := strconv.Atoi(s)
	return num
}

func isEquationPossiblePt2(eq equation, current, index int) bool {
	if current > eq.testValue {
		return false
	}

	num := eq.operands[index]
	if index == 0 {
		return isEquationPossiblePt2(eq, num, 1)
	}
	if index == len(eq.operands)-1 {
		return (current+num == eq.testValue) || (current*num == eq.testValue) || (concatInts(current, num) == eq.testValue)
	}

	return isEquationPossiblePt2(eq, current+num, index+1) || isEquationPossiblePt2(eq, current*num, index+1) || isEquationPossiblePt2(eq, concatInts(current, num), index+1)
}

func isEquationPossiblePt1(eq equation, current, index int) bool {
	if current > eq.testValue {
		return false
	}

	num := eq.operands[index]
	if index == 0 {
		return isEquationPossiblePt1(eq, num, 1)
	}
	if index == len(eq.operands)-1 {
		return (current+num == eq.testValue) || (current*num == eq.testValue)
	}

	return isEquationPossiblePt1(eq, current+num, index+1) || isEquationPossiblePt1(eq, current*num, index+1)
}

func parseEquation(s string) equation {
	parts := strings.Split(s, ":")

	testValue, _ := strconv.Atoi(strings.TrimSpace(parts[0]))
	operands, _ := utils.SplitStringToIntSlice(strings.TrimSpace(parts[1]), " ")

	return equation{testValue, operands}
}

func solvePart1(lines []string) int {
	defer utils.Timer("day7p1")()

	total := 0
	for _, line := range lines {
		equation := parseEquation(line)
		if isEquationPossiblePt1(equation, 0, 0) {
			total += equation.testValue
		}

	}
	return total
}

func solvePart2(lines []string) int {
	defer utils.Timer("day7p2")()

	total := 0
	for _, line := range lines {
		equation := parseEquation(line)
		if isEquationPossiblePt2(equation, 0, 0) {
			total += equation.testValue
		}

	}
	return total
}

func main() {
	fmt.Println("AoC 2024 - Day 7")
	fmt.Println("==================")

	lines, err := utils.ReadLines("day07/day07_input.txt")

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}
	// utils.PrintRows(lines)

	totalPart1 := solvePart1(lines)
	fmt.Println("Total Calibration Result (Part 1 Solution):", totalPart1)

	totalPart2 := solvePart2(lines)
	fmt.Println("Total Calibration Result with Concats (Part 2 Solution):", totalPart2)
}
