// AoC 2024 Day 02
package main

import (
	"aoc/utils"
	"fmt"
	"strconv"
	"strings"
)

func splitStringToIntSlice(s string) ([]int, error) {
	parts := strings.Split(s, " ")

	result := make([]int, 0, len(parts))

	for _, part := range parts {
		i, err := strconv.Atoi(part)
		if err != nil {
			return nil, fmt.Errorf("Error parsing string '%q' to int: %v", part, err)
		}
		result = append(result, i)
	}
	return result, nil
}

func parseReports(lines []string) [][]int {
	var reports [][]int

	for _, line := range lines {
		report, err := splitStringToIntSlice(line)

		if err != nil {
			fmt.Println("Error parsing report:", err)
			continue
		}
		reports = append(reports, report)
	}

	return reports
}

func isMonotone(report []int) bool {
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

func isDeltaInRange(report []int) bool {
	for i := range len(report) - 1 {
		delta := utils.Abs(report[i+1] - report[i])
		if delta > 3 {
			return false
		}
	}
	return true
}

func checkIfReportIsSafePart1(report []int) bool {
	if !isMonotone(report) {
		return false
	}
	if !isDeltaInRange(report) {
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

func main() {
	fmt.Println("AoC 2024 - Day 2")
	fmt.Println("==================")

	lines, err := utils.ReadLines("day02/day02_input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	reports := parseReports(lines)
	fmt.Println("Safe reports (Part 1 Solution):", countSafeReportsPart1(reports))
	fmt.Println("Safe reports (Part 2 Solution):", countSafeReportsPart2(reports))
}
