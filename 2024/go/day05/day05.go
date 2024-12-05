// AoC 2024 Day 05
package main

import (
	"aoc/utils"
	"fmt"
	"slices"
)

func splitLinesIntoTwoSections(lines []string) ([]string, []string) {
	var rules []string

	i := 0
	for ; i < len(lines); i++ {
		if lines[i] == "" {
			break
		}
		rules = append(rules, lines[i])
	}

	var updates []string
	for i++; i < len(lines); i++ {
		updates = append(updates, lines[i])
	}

	return rules, updates
}

func parseRules(lines []string) map[int][]int {
	rules := make(map[int][]int)

	for i, line := range lines {
		rule, err := utils.SplitStringToIntSlice(line, "|")

		if err != nil {
			fmt.Println("Error parsing rule:", i, err)
			continue
		}

		if len(rule) != 2 {
			fmt.Println("Rule has wrong number of parts:", i, line)
			continue
		}

		rules[rule[1]] = append(rules[rule[1]], rule[0])
	}

	return rules
}

func parseUpdates(lines []string) [][]int {
	var updates [][]int
	for i, line := range lines {
		update, err := utils.SplitStringToIntSlice(line, ",")

		if err != nil {
			fmt.Println("Error parsing update:", i, err)
			continue
		}

		updates = append(updates, update)
	}
	return updates
}

func checkIfUpdateIsValid(update []int, rules map[int][]int) bool {
	updates := len(update)
	for updateIndex := 0; updateIndex < updates; updateIndex++ {
		page := update[updateIndex]

		rule := rules[page]
		// check if any of these pages are after it
		for postCheckIndex := updateIndex + 1; postCheckIndex < updates; postCheckIndex++ {
			pageToCheck := update[postCheckIndex]
			if slices.Contains(rule, pageToCheck) {
				// fmt.Println("CheckingAfterRules |", afterRule, "Page:", page, "pageToCheck:", pageToCheck)
				return false
			}
		}
	}
	return true
}

func getCorrectUpdates(updates [][]int, rules map[int][]int) [][]int {
	var correctUpdates [][]int
	for _, update := range updates {
		if checkIfUpdateIsValid(update, rules) {
			correctUpdates = append(correctUpdates, update)
		}
	}
	return correctUpdates
}

func getIncorrectUpdates(updates [][]int, rules map[int][]int) [][]int {
	var incorrectUpdates [][]int
	for _, update := range updates {
		if !checkIfUpdateIsValid(update, rules) {
			incorrectUpdates = append(incorrectUpdates, update)
		}
	}
	return incorrectUpdates
}

func fixUpdate(update []int, rules map[int][]int) []int {
	updates := len(update)
	fixedUpdate := make([]int, updates)
	_ = copy(fixedUpdate, update)

	for i := 0; i < updates-1; i++ {
		swapped := false
		for j := 0; j < updates-i-1; j++ {
			page := fixedUpdate[j]
			nextPage := fixedUpdate[j+1]

			rule := rules[page]

			if slices.Contains(rule, nextPage) {
				fixedUpdate[j] = nextPage
				fixedUpdate[j+1] = page
				swapped = true
			}
		}
		if swapped == false {
			break
		}
	}
	return fixedUpdate
}

func fixUpdates(updates [][]int, rules map[int][]int) [][]int {
	var fixedUpdates [][]int
	for _, update := range updates {
		fixedUpdate := fixUpdate(update, rules)
		fixedUpdates = append(fixedUpdates, fixedUpdate)
	}
	return fixedUpdates
}

func countMiddlePages(updates [][]int) int {
	total := 0
	for _, update := range updates {
		middlePageIndex := (len(update) / 2)
		total += update[middlePageIndex]
	}
	return total
}

func solvePart1(lines []string) int {
	ruleLines, updateLines := splitLinesIntoTwoSections(lines)

	rules := parseRules(ruleLines)
	updates := parseUpdates(updateLines)

	fmt.Println("Rules:", rules)
	fmt.Println("Updates:", updates)
	correctUpdates := getCorrectUpdates(updates, rules)
	fmt.Println("Correctly-ordered Updates:", correctUpdates)

	total := countMiddlePages(correctUpdates)
	return total
}

func solvePart2(lines []string) int {
	ruleLines, updateLines := splitLinesIntoTwoSections(lines)

	afterRules := parseRules(ruleLines)
	updates := parseUpdates(updateLines)

	incorrectUpdates := getIncorrectUpdates(updates, afterRules)
	fmt.Println("Incorrectly-ordered Updates:", incorrectUpdates)

	fixedUpdates := fixUpdates(incorrectUpdates, afterRules)

	total := countMiddlePages(fixedUpdates)
	return total
}

func main() {
	fmt.Println("AoC 2024 - Day 5")
	fmt.Println("==================")

	lines, err := utils.ReadLines("day05/day05_input.txt")

	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	totalPart1 := solvePart1(lines)
	totalPart2 := solvePart2(lines)

	fmt.Println("Sum of middle page numbers of correctly-ordered updates (Part 1 Solution):", totalPart1)
	fmt.Println("Sum of middle page numbers of fixed updates (Part 2 Solution):", totalPart2)
}
