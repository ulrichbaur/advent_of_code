// AoC 2024 Day 02: Red-Nosed Reports
package main

import (
	"aoc/utils"
	"fmt"
)

func parseReports(lines []string) [][]int {
	var reports [][]int

	for _, line := range lines {
		report, err := utils.SplitStringToIntSlice(line, " ")

		if err != nil {
			fmt.Println("Error parsing report:", err)
			continue
		}
		reports = append(reports, report)
	}

	return reports
}

func isReportMonotone(report []int) bool {
	if len(report) <= 1 {
		return true
	}

	isIncreasing := true
	isDecreasing := true

	for i := range len(report) - 1 {
		if report[i+1] == report[i] {
			isIncreasing = false
			isDecreasing = false
		} else if report[i+1] > report[i] {
			isIncreasing = false
		} else if report[i+1] < report[i] {
			isDecreasing = false
		}
		if !isIncreasing && !isDecreasing {
			return false
		}
	}

	return true
}

func isReportDeltaInRange(report []int) bool {
	for i := range len(report) - 1 {
		delta := utils.Abs(report[i+1] - report[i])
		if delta > 3 {
			return false
		}
	}
	return true
}

func checkIfReportIsSafePart1(report []int) bool {
	if !isReportMonotone(report) {
		return false
	}
	if !isReportDeltaInRange(report) {
		return false
	}
	return true
}

func countSafeReportsPart1(reports [][]int) int {
	var safeReports int
	for _, report := range reports {
		if checkIfReportIsSafePart1(report) {
			safeReports++
		}
	}
	return safeReports
}

func isReportTolerable(report []int) bool {
	for i := range len(report) {
		var fixedReport []int
		for j := range len(report) {
			if i != j {
				fixedReport = append(fixedReport, report[j])
			}
		}
		if checkIfReportIsSafePart1(fixedReport) {
			return true
		}
	}

	return false
}

func countSafeReportsPart2(reports [][]int) int {
	var safeReports int
	for _, report := range reports {
		if checkIfReportIsSafePart1(report) || isReportTolerable(report) {
			safeReports++
		}
	}
	return safeReports
}

func solvePart1(lines []string) int {
	defer utils.Timer("day02p1")()

	totalReports := parseReports(lines)
	safeReportCount := countSafeReportsPart1(totalReports)
	return safeReportCount
}

func solvePart2(lines []string) int {
	defer utils.Timer("day02p2")()

	totalReports := parseReports(lines)
	safeReportCount := countSafeReportsPart2(totalReports)
	return safeReportCount
}

func main() {
	fmt.Println("AoC 2024 - Day 02: Red-Nosed Reports")
	fmt.Println("====================================")

	lines, err := utils.ReadLines("day02/day02_input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	safeReportCountPart1 := solvePart1(lines)
	fmt.Println("Safe reports (Part 1 Solution):", safeReportCountPart1)

	safeReportCountPart2 := solvePart2(lines)
	fmt.Println("Safe reports within tolerance (Part 2 Solution):", safeReportCountPart2)
}
