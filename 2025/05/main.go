package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

type Range struct {
	start int
	end   int
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Printf("Error opening input file: %v\n", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	ranges := []Range{}
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}
		var s, e int
		fmt.Sscanf(line, "%d-%d", &s, &e)
		ranges = append(ranges, Range{s, e})
	}

	sort.Slice(ranges, func(i, j int) bool {
		return ranges[i].start < ranges[j].start
	})

	results := 0
	max := 0
	for _, r := range ranges {
		if r.end > max {
			if r.start <= max {
				r.start = max + 1
			}
			results += r.end - r.start + 1
			max = r.end
		}
	}
	fmt.Println(results)
}
