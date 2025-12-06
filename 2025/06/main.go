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

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	var starts []int
	for i := 0; i < len(lines[len(lines)-1]); i++ {
		switch lines[len(lines)-1][i] {
		case '*':
			starts = append(starts, i)
		case '+':
			starts = append(starts, i)
		}
	}
	ranges := [][]int{}
	for i := range len(lines) - 1 {
		if len(ranges) < len(starts) {
			need := len(starts) - len(ranges)
			ranges = append(ranges, make([][]int, need)...)
		}

		for j, start := range starts {
			line := []byte(lines[i])
			var d []byte
			if j == len(starts)-1 {
				d = line[start:]
			} else {
				d = line[start : starts[j+1]-1]
			}

			if len(ranges[j]) < len(d) {
				need := len(d) - len(ranges[j])
				ranges[j] = append(ranges[j], make([]int, need)...)
			}

			for k, r := range d {
				if r != ' ' {
					ranges[j][k] = ranges[j][k]*10 + int(r-'0')
				}
			}
		}
	}
	result := 0
	for i, start := range starts {
		d := lines[len(lines)-1][start]
		switch d {
		case '+':
			result += add(ranges[i])
			continue
		case '*':
			result += mult(ranges[i])
			continue
		}
	}
	fmt.Println(result)
}

func mult(a []int) int {
	r := 1
	for _, i := range a {
		r *= i
	}
	return r
}

func add(a []int) int {
	r := 0
	for _, i := range a {
		r += i
	}
	return r
}
