// AMY: terminal companion
package main

import (
	"fmt"
	"os"
	"time"

	"github.com/ddmin/amy/games"
)

var version = "v1.1"

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
		scrollPrint(i, 45)
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
		fmt.Println("]  ✓ DONE")
	}
}

type game struct {
	name string
	game func()
}

func exit() {
	scrollPrint("See you later!", 40)
	os.Exit(0)
}

func main() {
	gameList := []game{
		{"Guessing Game", games.GuessingGame},
		{"Hangman", games.Hangman},
		{"Exit", exit},
	}

	printBoot := true
	// cool animation
	if printBoot {
		boot()
		fmt.Println()
		printTitle()
		time.Sleep(time.Second)
	}

	for true {
		fmt.Println()
		scrollPrint("Do you want to play a game? [y/n]", 40)
		response := games.GetUserString()
		fmt.Println()

		if string(response[0]) != "y" && string(response[0]) != "Y" {
			exit()
		}

		scrollPrint("Which game do you want to play?", 40)
		fmt.Println()

		for i, v := range gameList {
			fmt.Printf("%d. %s\n", i+1, v.name)
		}

		input := games.GetUserInt()

		if input == 0 {
			input = len(gameList)
		}

		currGame := gameList[input-1]
		fmt.Println()
		currGame.game()
	}
}
