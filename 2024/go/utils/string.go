// Package utils contains AoC utilities that can be reused between days
package utils

import "fmt"

// PrintRows prints a slice of strings row by row.
func PrintRows(rows []string) {
	for _, row := range rows {
		fmt.Println(row)
	}
}
