package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

const MAX_VALUE = 100

type State struct {
	Pos  int
	Hits int
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Printf("Error opening input file: %v\n", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	state1 := State{Pos: 50, Hits: 0}
	state2 := State{Pos: 50, Hits: 0}

	for scanner.Scan() {
		line := scanner.Text()
		state1 = state1.processLineStep1(line)
		state2 = state2.processLineStep2(line)
	}

	if err := scanner.Err(); err != nil {
		fmt.Printf("Scanner error: %v\n", err)
		return
	}

	fmt.Println(state1.Hits)
	fmt.Println(state2.Hits)
}

func (s State) processLineStep1(inputLine string) State {
	direction := inputLine[0]
	steps, err := strconv.Atoi(inputLine[1:])
	if err != nil {
		panic(fmt.Sprintf("Error parsing the value, %d", err))
	}

	switch direction {
	case 'L':
		s.Pos -= steps
		s.Pos %= MAX_VALUE
		if s.Pos < 0 {
			s.Pos += MAX_VALUE
		}
	case 'R':
		s.Pos += steps
		s.Pos %= MAX_VALUE
	}
	if s.Pos == 0 {
		s.Hits++
	}
	return s
}

func (s State) processLineStep2(line string) State {
	dir := line[0]
	n, err := strconv.Atoi(line[1:])
	if err != nil {
		panic(fmt.Sprintf("Error parsing the value, %d", err))
	}

	loops := n / MAX_VALUE
	step := n % MAX_VALUE

	pos := s.Pos
	hits := s.Hits + loops

	switch dir {
	case 'L':
		pos -= step
		if pos < 0 {
			hits++
			pos += MAX_VALUE
		}
	case 'R':
		pos += step
		if pos > MAX_VALUE {
			hits++
			pos -= MAX_VALUE
		}
	}
	if pos == 0 {
		hits++
	}
	return State{Pos: pos, Hits: hits}
}

func readLines(path string) ([]string, error) {
	file, err := os.Open(path)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

func RunStep1(path string) int {
	lines, _ := readLines(path)
	s := State{Pos: 50, Hits: 0}
	for _, line := range lines {
		s = s.processLineStep1(line)
	}
	return s.Hits
}

func RunStep2(path string) int {
	lines, _ := readLines(path)
	s := State{Pos: 50, Hits: 0}
	for _, line := range lines {
		s = s.processLineStep2(line)
	}
	return s.Hits
}
