package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

func check(e error) {
	if e != nil {
		log.Panic(e)
	}
}

func LoadInput(fileName string) ([]string, error) {
	content, err := os.ReadFile(fileName)
	if err != nil {
		return nil, err
	}

	lines := strings.Split(string(content), "\n")
	return lines, nil
}

func Part1(data []string) int {
	current_max := 0
	current := 0

	for i := 0; i < len(data); i++ {
		switch {
		case data[i] == "":
			current_max = max(current, current_max)
			current = 0
		default:
			j, err := strconv.Atoi(data[i])
			check(err)
			current += j
		}
	}

	return current_max
}

func Part2(data []string) int {
	current_maxes := []int{0, 0, 0}
	current := 0

	for i := 0; i < len(data); i++ {
		switch {
		case data[i] == "":
			updateTop3(current, &current_maxes)
			current = 0
		default:
			j, err := strconv.Atoi(data[i])
			check(err)
			current += j
		}
	}
	sum := 0
	for _, value := range current_maxes {
		sum += value
	}

	return sum
}

func updateTop3(value int, top3 *[]int) {
	*top3 = append(*top3, value)
	sort.Sort(sort.Reverse(sort.IntSlice(*top3)))
	*top3 = (*top3)[:min(3, len(*top3))]
}

func main() {
	startTime := time.Now()
	data, err := LoadInput("input.txt")
	check(err)
	fmt.Printf("Part 1: %v\n", Part1(data))
	fmt.Println(time.Since(startTime))
	fmt.Printf("Part 2: %v\n", Part2(data))
	fmt.Println(time.Since(startTime))

}
