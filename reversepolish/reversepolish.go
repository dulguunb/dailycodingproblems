// Daily coding problem #1857
package reversepolish

import (
	"fmt"
	"slices"
	"strconv"
)

func do(a int, b int, op string) int {
	if op == "+" {
		return a + b
	}
	if op == "-" {
		return a - b
	}
	if op == "*" {
		return a * b
	}
	if op == "/" {
		return a / b
	}
	return -1
}
func ReversePolish(symbols []string) (int, error) {
	// [15, 7, 1, 1, '+', '-', '/', 3, '*', 2, 1, 1, '+', '+', '-']
	// 15,7,1,1
	// +
	// 15,7,2
	// -
	// 15,5
	// /
	// 3,3
	// *
	// 9,2,1,1
	// +
	// 9,2,2
	// +
	// 9,4
	// 5
	operators := []string{"+", "-", "/", "*"}
	numbers := make([]int, 0)
	first, err := strconv.Atoi(symbols[0])
	if err != nil {
		return -1, fmt.Errorf("first element in the symbols array must be number")
	}
	i := 1
	numbers = append(numbers, first)
	for {
		if i == len(symbols) {
			return numbers[0], nil
		}
		if slices.Contains(operators, symbols[i]) {
			numbersI := len(numbers) - 1
			newNum := do(numbers[numbersI-1], numbers[numbersI], symbols[i])
			numbers = slices.Delete(numbers, numbersI-1, numbersI+1)
			numbers = append(numbers, newNum)
		}

		num, err := strconv.Atoi(symbols[i])
		if err != nil {

		} else {
			numbers = append(numbers, num)
		}
		i += 1
	}
}
