package main

import (
	"bufio"
	"log"
	"os"
)

func mod(a, b int64) int64 {
	return (a%b + b) % b
}

func part1() {
	file, err := os.Open("./inputs/aoc1.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scan := bufio.NewScanner(file)
	var sta uint = 50
	var res = 0
	for scan.Scan() {
		line := scan.Text()
		var pr uint = 1
		var a = len(line)
		if line[0] == 'L' {
			pr = 99
		}
		var nu uint = 0
		for i := range a - 1 {
			nu = nu*10 + uint(line[i+1]-'0')
		}
		sta = (sta + pr*nu) % 100
		if sta == 0 {
			res += 1
		}
	}
	println("Ends at zero = ", res)
	println("Ends at =", sta)
}

func part2() {
	file, err := os.Open("./inputs/aoc1.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scan := bufio.NewScanner(file)
	var sta int64 = 50
	var zer int64 = 0

	for scan.Scan() {
		line := scan.Text()
		var pr int64 = 1
		var a = len(line)
		if line[0] == 'L' {
			pr = -1
		}
		var nu int64 = 0
		for i := range a - 1 {
			nu = nu*10 + int64(line[i+1]-'0')
		}
		var roa = pr * nu
		var sta1 = sta + roa
		if sta1 == 0 {
			zer += 1

		} else if sta1 >= 100 {
			var sta2 = sta1 / 100
			zer += sta2

		} else if sta1 < 0 {
			var sta2 = (-sta1 + 100) / 100
			zer += sta2
			if sta == 0 {
				zer -= 1
			}
		}
		sta = mod(sta1, 100)
	}
	println("Ends at zero = ", zer)
	println("Ends at =", sta)
}

func main() {
	println("--- Part 1 ---")
	part1()
	println("--- Part 2 ---")
	part2()

}
