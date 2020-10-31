package games

import (
	"io/ioutil"
	"math/rand"
	"strings"
	"time"
)

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
