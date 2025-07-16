package main

import "fmt"

func findXmasDir(input []string, row, col int, dir string) (found bool) {
	findChars := "XMAS"
	findCharIndex := 0

	switch dir {
	case "up":
		if (row - 4) < 0 {
			return false
		}
		for curRow := row; curRow > row-4; curRow-- {
			if input[curRow][col] != findChars[findCharIndex] {
				return false
			}

			findCharIndex += 1
		}
		return true
	case "down":
		if row+4 > len(input)-1 {
			return false
		}
		for curRow := row; curRow < row+4; curRow++ {
			if input[curRow][col] != findChars[findCharIndex] {
				return false
			}

			findCharIndex += 1
		}
		return true
	case "left":
		if (col - 4) < 0 {
			return false
		}
		for curCol := col; curCol > col-4; curCol-- {
			if input[row][curCol] != findChars[findCharIndex] {
				return false
			}

			findCharIndex += 1
		}
		return true
	case "right":
		if (col + 4) > len(input[0])-1 {
			return false
		}
		for curCol := col; curCol < col+4; curCol++ {
			if input[row][curCol] != findChars[findCharIndex] {
				return false
			}

			findCharIndex += 1
		}
		return true
	case "ul":
		curCol := col
		curRow := row
		if (col-3) < 0 || (row-3) < 0 {
			return false
		}
		for findCharIndex := range 4 {
			if input[curRow][curCol] != findChars[findCharIndex] {
				return false
			}

			curCol -= 1
			curRow -= 1
		}
		return true
	case "ur":
		curCol := col
		curRow := row
		if (col+3) > len(input[0])-1 || (row-3) < 0 {
			return false
		}
		for findCharIndex := range 4 {
			if input[curRow][curCol] != findChars[findCharIndex] {
				return false
			}

			curCol += 1
			curRow -= 1
		}
		return true
	case "dl":
		curCol := col
		curRow := row
		if (col-3) < 0 || (row+3) > len(input)-1 {
			return false
		}
		for findCharIndex := range 4 {
			if input[curRow][curCol] != findChars[findCharIndex] {
				return false
			}

			curCol -= 1
			curRow += 1
		}
		return true
	case "dr":
		curCol := col
		curRow := row
		if (col+3) > len(input[0])-1 || (row+3) > len(input)-1 {
			return false
		}
		for findCharIndex := range 4 {
			if input[curRow][curCol] != findChars[findCharIndex] {
				return false
			}

			curCol += 1
			curRow += 1
		}
		return true
	}

	return
}

func findXmas(input []string) (totalXmas int) {
	directions := []string{
		"up",
		"down",
		"left",
		"right",
		"ul",
		"ur",
		"dl",
		"dr",
	}

	rows := len(input)
	cols := len(input[0])

	for row := 0; row < rows; row++ {
		for col := 0; col < cols; col++ {
			for _, direction := range directions {
				if findXmasDir(input, row, col, direction) {
					fmt.Printf("Row: %d, Col: %d, Dir: %s\n", row+1, col, direction)
					totalXmas += 1
				}
			}
		}
	}

	return
}
