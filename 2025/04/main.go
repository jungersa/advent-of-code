package main

import (
	"bufio"
	"fmt"
	"os"
)

type Pos struct {
	x int
	y int
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		fmt.Printf("Error opening input file: %v\n", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var grid [][]rune
	for scanner.Scan() {
		line := scanner.Text()
		if line != "" {
			grid = append(grid, []rune(line))
		}
	}

	result := 0
	for {
		toBeRemoved := []Pos{}
		for x := 0; x < len(grid[0]); x++ {
			for y := 0; y < len(grid); y++ {
				if grid[y][x] == '@' && processElement(x, y, grid) {
					toBeRemoved = append(toBeRemoved, Pos{x, y})
					result++
				}
			}
		}
		if len(toBeRemoved) == 0 {
			break
		}
		for _, toBeRemove := range toBeRemoved {
			grid[toBeRemove.y][toBeRemove.x] = '.'
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Printf("Scanner error: %v\n", err)
		return
	}
	fmt.Println(result)
}

func processElement(x, y int, grid [][]rune) bool {
	dirs := [][2]int{
		{-1, -1}, {-1, 0}, {-1, 1},
		{0, -1}, {0, 1},
		{1, -1}, {1, 0}, {1, 1},
	}
	rols := 0
	for _, dir := range dirs {
		checkX, chechkY := (x + dir[0]), (y + dir[1])
		if checkX >= 0 && checkX < len(grid[0]) && chechkY >= 0 && chechkY < len(grid) {
			if grid[chechkY][checkX] == '@' {
				rols++
			}
		}
	}
	return rols < 4
}
