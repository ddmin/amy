package games

import (
	"bufio"
	"fmt"
	"io/ioutil"
	"math/rand"
	"os"
	"strconv"
	"strings"
	"time"
)

type Result struct {
	Points    int
	PlayerWin bool
}

func PseudoRandom(lo, hi int) int {
	s1 := rand.NewSource(time.Now().UnixNano())
	r1 := rand.New(s1)
	return lo + r1.Intn(hi-lo+1)
}

func RandomWord() string {
	dat, err := ioutil.ReadFile("./files/words.txt")
	if err != nil {
		panic(err)
	}

	words := strings.Fields(string(dat))
	return words[PseudoRandom(0, len(words)-1)]
}

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
