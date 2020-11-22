// AMY: terminal companion
package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
	"time"

	"github.com/ddmin/amy/games"
)

// configs
var version = "v1.2"
var printBoot = true
var scrollSpeed = time.Duration(30)

// global game list
var gameList = []Game{
	{"Guessing Game", games.GuessingGame},
	{"Hangman", games.Hangman},
	{"Poison", games.Poison},
}

// ╭─────╮
// │ msg │
// ╰─────╯
func BoxEnclose(msg string) string {
	str := ""
	str += fmt.Sprintf("╭─%s─╮\n", strings.Repeat("─", len(msg)))
	str += fmt.Sprintf("│ %s │\n", msg)
	str += fmt.Sprintf("╰─%s─╯", strings.Repeat("─", len(msg)))
	return str
}

func scrollPrint(s string, t time.Duration) {
	for _, c := range s {
		time.Sleep(t * time.Millisecond)
		fmt.Print(string(c))
	}
	fmt.Println()
}

func printTitle() {
	time.Sleep(500 * time.Millisecond)

	amy := [...]string{
		"  __ _ _ __ ___  _   _ ",
		" / _` | '_ ` _ \\| | | |",
		"| (_| | | | | | | |_| |",
		" \\__,_|_| |_| |_|\\__, |",
		"                 |___/  " + version,
		"",
	}

	for _, i := range amy {
		scrollPrint(i, 20)
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
			sleep := games.PseudoRandom(0, 40)
			time.Sleep(time.Duration(sleep) * time.Millisecond)
		}
		fmt.Println("]  ✓ DONE")
	}
}

type Player struct {
	name    string
	points  int
	history map[string]*Score
}

func LoadPlayer() Player {
	// initializing variables
	player := Player{}
	player.history = map[string]*Score{}

	for _, game := range gameList {
		player.history[game.name] = &Score{0, 0}
	}

	dat, err := ioutil.ReadFile("./files/player.data")

	if err != nil {
		scrollPrint("Hello! I am AMY. What's your name?", scrollSpeed)
		name := games.GetUserString()
		player.name = name

		fmt.Println()
		scrollPrint(fmt.Sprintf("Nice to meet you %s!", player.name), scrollSpeed)

		return player
	} else {
		info := string(dat)
		data := strings.Split(info, "\n")

		filtered := make([]string, 0, 10)

		for _, d := range data {
			if d != "" {
				filtered = append(filtered, d)
			}
		}

		for i := range filtered {
			if filtered[i][0] == '[' {
				switch filtered[i] {
				case "[name]":
					player.name = filtered[i+1]
				case "[points]":
					points, _ := strconv.Atoi(filtered[i+1])
					player.points = points
				default:
					fieldName := filtered[i][1 : len(filtered[i])-1]
					strScore := strings.Fields(filtered[i+1][1 : len(filtered[i+1])-1])
					win, _ := strconv.Atoi(strScore[0])
					lose, _ := strconv.Atoi(strScore[1])

					score := Score{
						Win:  win,
						Lose: lose,
					}

					player.history[fieldName] = &score
				}
			}
		}
		scrollPrint(fmt.Sprintf("Welcome back %s!", player.name), scrollSpeed)
		return player
	}
}

func (p Player) DisplayPoints() {
	fmt.Println(BoxEnclose(fmt.Sprintf("%s's Points: %v", p.name, p.points)))
}

func (p Player) SaveFile() {
	template := ""

	template += fmt.Sprintf("[name]\n%s\n\n", p.name)
	template += fmt.Sprintf("[points]\n%v\n\n", p.points)

	for _, game := range gameList {
		template += fmt.Sprintf("[%s]\n%v\n\n", game.name, *p.history[game.name])
	}

	ioutil.WriteFile("./files/player.data", []byte(template), 0644)
}

type Game struct {
	name string
	game func() (result games.Result)
}

type Score struct {
	Win  int
	Lose int
}

func exit() games.Result {
	scrollPrint("See you later!", scrollSpeed)
	os.Exit(0)
	return games.Result{}
}

func main() {
	// cool animation
	if printBoot {
		boot()
		fmt.Println()
		printTitle()
		time.Sleep(time.Second)
	}

	player := LoadPlayer()

	for true {
		fmt.Println()

		scrollPrint("What do you want to do?", scrollSpeed)
		fmt.Println()

		otherOptions := []Game{
			{"Exit", exit},
		}
		gameOptions := append(gameList, otherOptions...)

		for i, v := range gameOptions {
			fmt.Printf("%d. %s\n", i+1, v.name)
		}

		input := games.GetUserInt()

		if input == 0 {
			input = len(gameOptions)
		}

		currGame := gameOptions[input-1]
		fmt.Println()

		result := currGame.game()
		fmt.Println()

		// update player score
		if result.PlayerWin {
			fmt.Printf("%s earned %v points!\n", player.name, result.Points)
			player.history[currGame.name].Win += 1
			player.points += result.Points
		} else {
			fmt.Printf("%s lost %v points!\n", player.name, result.Points)
			player.history[currGame.name].Lose += 1
			player.points -= result.Points
		}

		player.SaveFile()
		player.DisplayPoints()
	}
}
