package games

import (
	"math/rand"
	"time"
)

func PseudoRandom(lo, hi int) int {
	s1 := rand.NewSource(time.Now().UnixNano())
	r1 := rand.New(s1)
	return lo + r1.Intn(hi-lo+1)
}
