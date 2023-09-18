package main

import "fmt"

func main() {
	fmt.Println(linearSearch(40, []int{2, 2, 1}))
}

func linearSearch[Element comparable, Array []Element](element Element, arr Array) bool {
	for i := 0; i < len(arr); i++ {
		if arr[i] == element {
			return true
		}
	}

	return false
}
