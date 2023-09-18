package main

import "fmt"

func main() {
	fmt.Println(
		bubbleSort([]int{
			1, 3, 5, 2, 8, -1,
		}),
	)
}

func bubbleSort(array []int) []int {
	sorted := array[0:]

	for i := len(array) - 1; i > 0; i-- {
		for j := 0; j < i; j++ {
			current := array[j]
			next := array[j+1]

			if current > next {
				sorted[j] = next
				sorted[j+1] = current
			}

		}
	}

	return sorted
}
