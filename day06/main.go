package main

import (
	"bufio"
	"fmt"
	"os"
)

func turnRight(playerDir rune) rune {
	if playerDir == '^' {
		return '>'
	} else if playerDir == 'v' {
		return '<'
	} else if playerDir == '<' {
		return '^'
	} else if playerDir == '>' {
		return 'v'
	}

	return playerDir
}

func getNextPos(playerPos [2]int, playerDir rune) [2]int {
	var nextPos [2]int
	if playerDir == '^' {
		nextPos = [2]int{playerPos[0], playerPos[1] - 1}
	} else if playerDir == 'v' {
		nextPos = [2]int{playerPos[0], playerPos[1] + 1}
	} else if playerDir == '<' {
		nextPos = [2]int{playerPos[0] - 1, playerPos[1]}
	} else if playerDir == '>' {
		nextPos = [2]int{playerPos[0] + 1, playerPos[1]}
	}

	return nextPos
}

func walk(labMap [][]rune, playerPos [2]int, playerDir rune, numRows int, numCols int, visits int) int {
	// calculate next position
	var nextPos [2]int = getNextPos(playerPos, playerDir)

	// check if next position is out of bounds
	if nextPos[0] < 0 || nextPos[0] >= numCols || nextPos[1] < 0 || nextPos[1] >= numRows {
		return visits
	}

	// check if next position is an obstacle
	if labMap[nextPos[1]][nextPos[0]] == '#' {
		playerDir = turnRight(playerDir)
		return walk(labMap, playerPos, playerDir, numRows, numCols, visits)
	}

	// move to next position
	playerPos = nextPos

	if labMap[playerPos[1]][playerPos[0]] == '.' {
		visits++
	}

	labMap[playerPos[1]][playerPos[0]] = 'X'

	return walk(labMap, playerPos, playerDir, numRows, numCols, visits)
}

func paradoxWalk(labMap [][]rune, playerPos [2]int, playerDir rune, numRows int, numCols int, visits int, iterations int) int {
	// this is our assumption of a paradox (getting stuck in a loop)
	if iterations > numCols*numRows {
		return 1
	}

	// calculate next position
	var nextPos [2]int = getNextPos(playerPos, playerDir)

	// check if next position is out of bounds
	if nextPos[0] < 0 || nextPos[0] >= numCols || nextPos[1] < 0 || nextPos[1] >= numRows {
		return -1
	}

	// check if next position is an obstacle
	if labMap[nextPos[1]][nextPos[0]] == '#' || labMap[nextPos[1]][nextPos[0]] == 'O' {
		labMap[playerPos[1]][playerPos[0]] = '+'
		playerDir = turnRight(playerDir)
		return paradoxWalk(labMap, playerPos, playerDir, numRows, numCols, visits, iterations)
	}

	// move to next position
	playerPos = nextPos

	// increment iterations count only when player moves
	iterations++

	if playerDir == '^' || playerDir == 'v' {
		labMap[playerPos[1]][playerPos[0]] = '|'
	} else {
		labMap[playerPos[1]][playerPos[0]] = '-'
	}

	return paradoxWalk(labMap, playerPos, playerDir, numRows, numCols, visits, iterations)
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Please provide a filename")
		return
	}

	filename := os.Args[1]

	data, err := os.Open(filename)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer data.Close()

	scanner := bufio.NewScanner(data)
	var originalMap [][]rune
	for scanner.Scan() {
		line := scanner.Text()
		var chars []rune

		for _, char := range line {
			chars = append(chars, char)
		}

		originalMap = append(originalMap, chars)
	}

	if err := scanner.Err(); err != nil {
		fmt.Println(err)
		return
	}

	var playerPos [2]int
	var playerDir rune
	var numRows int = len(originalMap)
	var numCols int = len(originalMap[0])

	var visitsMap = make([][]rune, numRows)
	for i := range originalMap {
		visitsMap[i] = make([]rune, numCols)
		copy(visitsMap[i], originalMap[i])
	}

	var paradoxMap = make([][]rune, numRows)
	for i := range originalMap {
		paradoxMap[i] = make([]rune, numCols)
		copy(paradoxMap[i], originalMap[i])
	}

	// start by finding out player position
	for y := 0; y < numRows; y++ {
		for x := 0; x < numCols; x++ {
			if visitsMap[y][x] == '^' || visitsMap[y][x] == 'v' || visitsMap[y][x] == '<' || visitsMap[y][x] == '>' {
				playerPos[0] = x
				playerPos[1] = y
				playerDir = visitsMap[y][x]
				break
			}
		}
	}

	visits := walk(visitsMap, playerPos, playerDir, numRows, numCols, 1)

	var numParadoxes int

	for y := 0; y < numRows; y++ {
		for x := 0; x < numCols; x++ {
			if visitsMap[y][x] == 'X' {
				// reset paradox map each time
				for i := range originalMap {
					copy(paradoxMap[i], originalMap[i])
				}

				// place O
				paradoxMap[y][x] = 'O'

				if (paradoxWalk(paradoxMap, playerPos, playerDir, numRows, numCols, 0, 0) != -1) {
					numParadoxes++
				}
			}
		}
	}

	fmt.Printf("Number of paradoxes: %d\n", numParadoxes)
	fmt.Printf("Visits: %d\n", visits)
	fmt.Printf("Num rows: %d, Num cols: %d\n", numRows, numCols)
}
