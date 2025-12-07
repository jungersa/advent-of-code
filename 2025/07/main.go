package main

import (
	"bufio"
	"fmt"
	"os"
)

type Key struct {
	P      int
	LineId int
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Printf("Error opening input file: %v\n", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	scanner.Scan()
	firstLine := scanner.Text()

	tachion := 0
	for s := range firstLine {
		if firstLine[s] == 'S' {
			tachion = s
			break
		}
	}
	var lines []string
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	memory := make(map[Key]int)
	fmt.Println(splitter(tachion, 1, lines, memory))
}

func splitter(p int, lineId int, lines []string, memory map[Key]int) int {
	if lineId >= len(lines) {
		return 1
	}
	if p > len(lines[lineId]) || p < 0 {
		return 0
	}
	key := Key{p, lineId}

	if val, ok := memory[key]; ok {
		return val
	}

	var r int
	switch lines[lineId][p] {
	case '^':
		r = splitter(p-1, lineId+1, lines, memory) +
			splitter(p+1, lineId+1, lines, memory)
	default:
		r = splitter(p, lineId+1, lines, memory)
	}

	memory[key] = r
	return r
}
