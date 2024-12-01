// Package utils contains AoC utilities that can be reused between days
package utils

// Abs returns the absolute value of the provided integer.
func Abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

// Sum sums up a slice of integers.
func Sum(nums []int) int {
	var total int
	for _, num := range nums {
		total += num
	}
	return total
}
