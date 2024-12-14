// Package utils contains AoC utilities that can be reused between days
package utils

import (
	"fmt"
	"time"
)

// Timer can be used to how long a function takes to compute by using the timer in a `defer` call at the start of a function.
func Timer(name string) func() {
	start := time.Now()
	return func() {
		fmt.Printf("%s took %v\n", name, time.Since(start))
	}
}
