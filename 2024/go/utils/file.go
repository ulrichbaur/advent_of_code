// Package utils contains AoC utilities that can be reused between days
package utils

import (
	"bufio"
	"fmt"
	"os"
)

// ReadLines reads all lines in a file into a slice of strings.
func ReadLines(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, fmt.Errorf("Error opening file %q: %v", filename, err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("Error reading file %q: %v", filename, err)
	}

	return lines, nil
}
