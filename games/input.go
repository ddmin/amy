package games

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func GetUserString() string {
	scanner := bufio.NewReader(os.Stdin)
	fmt.Print("> ")
	input, _ := scanner.ReadString('\n')
	input = strings.Replace(input, "\n", "", -1)
	return input
}

func GetUserInt() int {
	input := GetUserString()
	guess, _ := strconv.Atoi(input)
	return guess
}
