package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Printf("Error opening input file: %v\n", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	result := 0
	for scanner.Scan() {
		line := scanner.Text()
		result += processLine(line)
	}

	if err := scanner.Err(); err != nil {
		fmt.Printf("Scanner error: %v\n", err)
		return
	}
	fmt.Println(result)
}

// func processLine(line string) int {
// 	var a, b int
// 	aPos := -1
// 	// 1. find the highest index and value before n-1
// 	for i := 0; i < len(line)-1; i++ {
// 		d := int(line[i] - '0')
// 		if i < len(line)-1 && d > a {
// 			aPos = i
// 			a = d
//          b=0
// 		}

// 		// if aPos != -1 && i > aPos && d > b {
// 		// 	b = d
// 		// }
// 	}
// 	// 2. find the highest value from i to n
// 	for i := aPos + 1; i < len(line); i++ {
// 		if int(line[i]-'0') > b {
// 			b = int(line[i] - '0')
// 		}
// 	}
// 	// 3. return ab
// 	return a*10 + b
// }

func processLine(line string) int {
	var a, result int
	aPos := -1
	//  find the highest index and value before n-12
	for i := range 12 {
		for j := aPos + 1; j < len(line)+i-11; j++ {
			d := int(line[j] - '0')
			if d > a {
				aPos = j
				a = d
			}
		}
		result = result*10 + a
		a = 0
	}
	return result
}
