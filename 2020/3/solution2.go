package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	slopes := [][2]int{
		{1, 0},
		{3, 0},
		{5, 0},
		{7, 0},
		{-1, 0},
	}

	s := bufio.NewScanner(f)
	for row := 0; s.Scan(); row++ {
		b := s.Bytes()
		width := len(b)
		if width == 0 {
			continue
		}

		for i, v := range slopes {
			col := 0
			if v[0] > 0 {
				col = v[0] * row
			} else if row % 2 == 0 {
				col = row / 2
			} else {
				continue
			}

			if b[col%width] == '#' {
				slopes[i][1]++
			}
		}
	}

	prod := 1
	for _, v := range slopes {
		prod *= v[1]
	}

	fmt.Println(prod)
}
