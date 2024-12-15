// AoC 2024 Day 03: Mull It Over
package main

import (
	"aoc/utils"
	"fmt"
	"strings"
)

const prefix = "mul("

func containsOnlyDigitsAndCommas(s string) bool {
	for _, char := range s {
		if char != ',' && !utils.IsDigit(char) {
			return false
		}
	}
	return true
}

func checkIfInstructionCandidateIsValid(candidate string) ([]int, error) {
	openingParens := 3
	closingParens := strings.Index(candidate, ")")
	if closingParens < (4+3) || closingParens > (4+7) {
		return nil, fmt.Errorf("No closing parenthesis found in candidate")
	}
	narrowedCandidate := candidate[openingParens+1 : closingParens]

	if !containsOnlyDigitsAndCommas(narrowedCandidate) {
		return nil, fmt.Errorf("Candidate contains forbidden characters")
	}

	if strings.Count(narrowedCandidate, ",") != 1 {
		return nil, fmt.Errorf("Candidate does not contain exactly one comma")
	}

	parts := strings.Split(narrowedCandidate, ",")
	if len(parts) != 2 {
		return nil, fmt.Errorf("Candidate does not contain exactly two numbers")
	}
	instruction, err := utils.SplitStringToIntSlice(narrowedCandidate, ",")
	if err != nil {
		return nil, fmt.Errorf("Error parsing instruction '%q' to int: %v", narrowedCandidate, err)
	}

	return instruction, nil
}

func getValidInstructions(candidates []string) [][]int {
	var validInstructions [][]int
	for _, candidate := range candidates {
		instruction, err := checkIfInstructionCandidateIsValid(candidate)
		if err != nil {
			continue
		}
		validInstructions = append(validInstructions, instruction)
	}
	return validInstructions
}

func getInstructionCandidates(text string) []string {
	var instructionCandidates []string
	linePos := 0
	substringPos := 0
	for substringPos >= 0 {
		substringPos = strings.Index(text[linePos:], prefix)
		if substringPos >= 0 {
			pos := linePos + substringPos
			// fmt.Println("Found possible mul instruction at position", pos, "Instruction Area:", line[pos:])
			instructionCandidates = append(instructionCandidates, text[pos:])
			linePos += substringPos + len(prefix)
		}
	}
	return instructionCandidates
}

func parseInstructionLinePart1(s string) [][]int {
	instructionCandidates := getInstructionCandidates(s)
	instructions := getValidInstructions(instructionCandidates)
	return instructions
}

func parseInstructionsPart1(lines []string) [][]int {
	concatLine := strings.Join(lines, "\n")
	instructions := parseInstructionLinePart1(concatLine)
	return instructions
}

// getEnabledSections returns a slice of start and stop positions inside the line
func getEnabledSections(line string) [][]int {
	var sections [][]int
	// first section: start with enabled - stops at first don't() or end of line
	linePos := 0
	startPos := 0
	for linePos < len(line) {
		endPos := strings.Index(line[linePos:], "don't()")
		if endPos == -1 {
			sections = append(sections, []int{linePos, len(line)})
			break
		}
		sections = append(sections, []int{linePos, linePos + endPos})
		startPos = strings.Index(line[linePos+endPos:], "do()")
		if startPos == -1 {
			break
		}
		linePos += endPos + startPos
	}
	return sections
}

func parseInstructionLinePart2(line string) [][]int {
	sections := getEnabledSections(line)

	var instructions [][]int
	for _, section := range sections {
		instructionCandidates := getInstructionCandidates(line[section[0]:section[1]])
		newInstructions := getValidInstructions(instructionCandidates)
		instructions = append(instructions, newInstructions...)
	}
	return instructions
}

func solvePart1(lines []string) int {
	defer utils.Timer("day03p1")()
	total := 0
	instructions := parseInstructionsPart1(lines)
	for _, instruction := range instructions {
		total += instruction[0] * instruction[1]
	}
	return total
}

func solvePart2(lines []string) int {
	defer utils.Timer("day03p2")()
	total := 0
	instructions := parseInstructionsPart2(lines)
	for _, instruction := range instructions {
		total += instruction[0] * instruction[1]
	}
	return total
}

func parseInstructionsPart2(lines []string) [][]int {
	line := strings.Join(lines, "\n")
	instructions := parseInstructionLinePart2(line)
	return instructions
}

func main() {
	fmt.Println("AoC 2024 - Day 03: Mull It Over")
	fmt.Println("===============================")

	lines, err := utils.ReadLines("day03/day03_input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}

	totalPart1 := solvePart1(lines)
	fmt.Println("Multiplication instruction total (Part 1 Solution):", totalPart1)

	totalPart2 := solvePart2(lines)
	fmt.Println("Multiplication instruction total (Part 2 Solution):", totalPart2)
}
