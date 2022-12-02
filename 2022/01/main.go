package main

import (
	_ "embed"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

//go:embed input.txt
var input string

func parse(input string) []int {
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

	return totals
}

func part1(totals []int) any {
	max := 0
	for _, total := range totals {
		if total > max {
			max = total
		}
	}

	return max
}

func part2(totals []int) any {
	sort.Ints(totals)

	fmt.Println(totals)
	most := totals[len(totals)-3:]
	fmt.Println(most)
	var final int
	for _, n := range most {
		final += n
	}
	return final
}

func main() {
	totals := parse(input)
	fmt.Printf("Part 1: %v\n", part1(totals))
	fmt.Printf("Part 2: %v\n", part2(totals))
}
