// AMY: terminal companion
package main

import (
	"fmt"
	"github.com/ddmin/amy/games"
	"time"
)

var version = "v1.0"

func scrollPrint(s string, t time.Duration) {
	for _, c := range s {
		time.Sleep(t * time.Millisecond)
		fmt.Print(string(c))
	}
	fmt.Println()
}

func printTitle() {
	scrollPrint("Loading AMY "+version+"...", 40)
	fmt.Println()
	time.Sleep(time.Second)

	amy := [...]string{
		"  __ _ _ __ ___  _   _ ",
		" / _` | '_ ` _ \\| | | |",
		"| (_| | | | | | | |_| |",
		" \\__,_|_| |_| |_|\\__, |",
		"                 |___/  " + version,
		"",
	}

	for _, i := range amy {
		scrollPrint(i, 80)
	}
}

func boot() {
	loading := [...]string{
		"Updating AMY kernel            ",
		"Syncing Remote Packages        ",
		"Installing Language Recognition",
		"Initializing Games             ",
		"Training Game AI               ",
	}

	for _, i := range loading {
		fmt.Print(i + " [")
		for s := 0; s < 20; s++ {
			fmt.Print("=")
			sleep := games.PseudoRandom(0, 100)
			time.Sleep(time.Duration(sleep) * time.Millisecond)
		}
		fmt.Println("]  DONE")
	}
}

func main() {
	// cool animation
	boot()
	fmt.Println()
	time.Sleep(500 * time.Millisecond)
	printTitle()

	// games.GuessingGame()
	games.Hangman()
}
