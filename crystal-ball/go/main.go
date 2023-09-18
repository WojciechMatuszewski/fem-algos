package main

import (
	"fmt"
	"math"
)

func main() {
	fmt.Println(
		crystalBall([]bool{false, false, false, false, true, true}),
	)
}

func crystalBall(breaks []bool) int {
	jumpAmount := int(math.Floor(math.Sqrt(float64(len(breaks)))))

	firstBreak := -1
	for i := jumpAmount; i <= len(breaks); i += jumpAmount {
		if breaks[i] {
			firstBreak = i
			break
		}
	}
	if firstBreak == -1 {
		return -1
	}

	checkFrom := firstBreak - jumpAmount
	for j := 0; j <= checkFrom; j++ {
		if breaks[j+checkFrom] {
			return j + checkFrom
		}
	}

	return -1

}
