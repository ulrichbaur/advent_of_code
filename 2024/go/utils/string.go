// Package utils contains AoC utilities that can be reused between days
package utils

import (
	"fmt"
	"strconv"
	"strings"
)

// IsDigit checks if character is a digit.
func IsDigit(c rune) bool {
	return c >= '0' && c <= '9'
}

// PrintRows prints a slice of strings row by row.
func PrintRows(rows []string) {
	for _, row := range rows {
		fmt.Println(row)
	}
}

// SplitStringToIntSlice splits a string into a slice of integers.
func SplitStringToIntSlice(s string, seperator string) ([]int, error) {
	parts := strings.Split(s, seperator)

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

// TransposeRows transforms a slice of strings (rows) to a slice of strings (columns) out of a grid of text.
// NOTE: a grid assumes that every string in the string slice has the same length!
func TransposeRows(gridRows []string) []string {
	columnLen := len(gridRows[0])
	columns := make([]string, columnLen)

	for colIndex := 0; colIndex < columnLen; colIndex++ {
		str := ""
		for _, row := range gridRows {
			str += string(row[colIndex])
		}
		columns[colIndex] = str
	}
	return columns
}

// CreateDiagonalsLeftToRight creates a diagonal from the bottom left to the top right out of a grid of text.
// NOTE: a grid assumes that every string in the string slice has the same length!
//
//	    x
//	  x
//	x
func CreateDiagonalsLeftToRight(gridRows []string) []string {
	rowLen := len(gridRows)
	columnLen := len(gridRows[0])

	diagonals := make([]string, 0)

	for k := 0; k < rowLen; k++ {
		diagonal := ""
		for rowIndex, colIndex := k, 0; rowIndex >= 0 && colIndex < columnLen; {
			diagonal += string(gridRows[rowIndex][colIndex])
			rowIndex--
			colIndex++
		}
		diagonals = append(diagonals, diagonal)
	}
	for k := 1; k < columnLen; k++ {
		diagonal := ""
		for rowIndex, colIndex := rowLen-1, k; rowIndex >= 0 && colIndex < columnLen; {
			diagonal += string(gridRows[rowIndex][colIndex])
			rowIndex--
			colIndex++
		}
		diagonals = append(diagonals, diagonal)
	}
	return diagonals
}

// CreateDiagonalsRightToLeft creates a diagonal from the top left to the bottom right out of a grid of text.
// NOTE: a grid assumes that every string in the string slice has the same length!
//
//	x
//	  x
//	    x
func CreateDiagonalsRightToLeft(gridRows []string) []string {
	rowLen := len(gridRows)
	columnLen := len(gridRows[0])

	diagonals := make([]string, 0)

	for k := rowLen - 1; k >= 0; k-- {
		diagonal := ""
		for rowIndex, colIndex := k, 0; rowIndex < rowLen && colIndex < columnLen; {
			diagonal += string(gridRows[rowIndex][colIndex])
			rowIndex++
			colIndex++
		}
		diagonals = append(diagonals, diagonal)
	}
	for k := 1; k < columnLen; k++ {
		diagonal := ""
		for rowIndex, colIndex := 0, k; rowIndex < rowLen && colIndex < columnLen; {
			diagonal += string(gridRows[rowIndex][colIndex])
			rowIndex++
			colIndex++
		}
		diagonals = append(diagonals, diagonal)
	}
	return diagonals
}
