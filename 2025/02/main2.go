package main

import (
	"bufio"
	"bytes"
	"fmt"
	"os"
	"strconv"
	"strings"
	"sync"
)

func main() {
	file, err := os.Open("test.txt")
	if err != nil {
		fmt.Printf("Error opening input file: %v\n", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	result := 0
	resultsCh := make(chan int)
	var wg sync.WaitGroup
	var wgCollector sync.WaitGroup

	wgCollector.Go(
		func() {
			for r := range resultsCh {
				result += r
			}
		})

	for scanner.Scan() {
		for id := range strings.SplitSeq(scanner.Text(), ",") {
			wg.Add(1)
			go func(id string) {
				defer wg.Done()
				resultsCh <- processLine(id)
			}(id)
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Printf("Scanner error: %v\n", err)
		return
	}

	wg.Wait()
	close(resultsCh)
	wgCollector.Wait()

	fmt.Println(result)
}

func processLine(line string) int {
	parts := strings.Split(line, "-")
	start, _ := strconv.Atoi(parts[0])
	end, _ := strconv.Atoi(parts[1])

	total := 0
	for i := start; i <= end; i++ {
		if isValidId(strconv.Itoa(i)) {
			total += i
		}
	}
	return total
}

// func isValidId(id string) bool {
// 	idBytes := []byte(id)
// 	if len(idBytes)%2 != 0 {
// 		return false // can't split evenly
// 	}

// 	mid := len(idBytes) / 2
// 	return bytes.Equal(idBytes[:mid], idBytes[mid:])
// }

func isValidId(id string) bool {
	idBytes := []byte(id)
	L := len(idBytes)
	for nSplits := 2; nSplits <= L; nSplits++ {
		if L%nSplits != 0 {
			continue // Not possible to have equal splits
		}
		split := L / nSplits
		ok := true
		for j := 0; j < nSplits-1; j++ {
			if !bytes.Equal(idBytes[split*j:split*(j+1)], idBytes[split*(j+1):split*(j+2)]) {
				ok = false
				break
			}
		}
		if ok {
			return true
		}
	}
	return false
}
