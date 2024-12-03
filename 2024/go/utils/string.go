// Package utils contains AoC utilities that can be reused between days
package utils

import (
	"fmt"
	"strconv"
	"strings"
)

// PrintRows prints a slice of strings row by row.
func PrintRows(rows []string) {
	for _, row := range rows {
		fmt.Println(row)
	}
}

// SplitStringToIntSlice splits a string into a slice of integers.
func SplitStringToIntSlice(s string) ([]int, error) {
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