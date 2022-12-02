package main

import (
	_ "embed"
	"fmt"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func part1() any {
	elves := strings.Split(input, "\n\n")

	var totals []int
	for _, elf := range elves {
		cals := strings.Split(elf, "\n")

		var total int
		for _, cal := range cals {
			n, err := strconv.Atoi(cal)
			if err != nil {
				panic(err)
			}

			total += n
		}

		totals = append(totals, total)
	}

	max := 0
	for _, total := range totals {
		if total > max {
			max = total
		}
	}

	return max
}

func main() {
	fmt.Printf("Part 1: %v\n", part1())
}
