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

	hits := 0
	s := bufio.NewScanner(f)
	for col := 0; s.Scan(); col += 3 {
		b := s.Bytes()
		width := len(b)
		if width == 0 {
			continue
		}
		if b[col%width] == '#' {
			hits++
		}
	}

	fmt.Println(hits)
}
