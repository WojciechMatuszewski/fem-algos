package main

import (
	"fmt"
	"math"
	"sort"
)

func main() {
	fmt.Println(binarySearch(2, []int{1, 2, 3, 4, 5, 6}))
}

func binarySearch(element int, array []int) bool {
	sorted := sort.IntSlice(array)

	low := 0
	high := sorted.Len() - 1

	for low <= high {
		mid := int(math.Floor(float64(low) + (float64(high-low))/2))

		value := array[mid]

		fmt.Println(
			"low", low,
			"mid", mid,
			"high", high,
			"value", value,
		)

		if element == value {
			return true
		}

		if element > value {
			low = mid + 1
		}

		if element < value {
			high = mid
		}

	}

	return false
}
