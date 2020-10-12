// AMY: terminal companion
package main

import (
	"fmt"
	"strings"

	"math/rand"
	"time"

	"bufio"
	"os"
	"strconv"
)

type Game struct {
	name     string
	function func()
}

func generate_random(low, high int) int64 {
	seed := rand.NewSource(time.Now().UnixNano())
	random := rand.New(seed)

	return int64(random.Intn(high)) + int64(low)
}

func hangman() {
	scanner := bufio.NewScanner(os.Stdin)

	// list of words to choose from
	words := []string{
		"apple",
		"banana",
		"cherry",
		"durian",
		"elderberry",
		"fig",
		"grape",
		"kiwi",
		"melon",
		"orange",
		"strawberry",
		"watermelon",
	}

	lives := 6

	unique := map[string]bool{}
	player_guesses := map[string]bool{}
	player_correct := []string{}

	word := words[generate_random(0, len(words))]

	for _, c := range word {
		unique[string(c)] = true
	}

	for lives > 0 && len(unique) != len(player_correct) {

		var blank string
		for _, l := range word {
			if _, ok := player_guesses[string(l)]; ok {
				blank += string(l) + " "
			} else {
				blank += "_ "
			}
		}

		fmt.Println(blank)

		fmt.Printf("Lives: %d\n", lives)

		var str_guesses string
		for i := range player_guesses {
			str_guesses += i + " "
		}
		fmt.Printf("Letters Guessed: %s\n", str_guesses)

		var letter string

		var _ bool
		already_guessed := true

		for letter == "" || len(letter) != 1 || (letter < "A" || letter > "z") || already_guessed {
			fmt.Print("Guess a letter: ")
			scanner.Scan()
			letter = scanner.Text()

			_, already_guessed = player_guesses[letter]
		}

		fmt.Println()

		if letter != "" && strings.Contains(word, letter) {
			fmt.Printf("Correct! '%s' is in the word.", letter)
			player_guesses[letter] = true
			player_correct = append(player_correct, letter)
		} else {
			fmt.Printf("Incorrect! '%s' is not in the word.", letter)
			player_guesses[letter] = true
			lives -= 1
		}

		fmt.Printf("\n\n")
	}

	var blank string
	for _, l := range word {
		blank += string(l) + " "
	}

	fmt.Println(blank)

	if lives == 0 {
		fmt.Println("You Lose...")
	} else if len(unique) == len(player_correct) {
		fmt.Println("You Win!")
	}
}

func guessing_game() {
	scanner := bufio.NewScanner(os.Stdin)

	fmt.Println("Try to guess my number!")
	fmt.Println()

	high := 100
	secret := generate_random(1, high)

	count := 0
	var middle int64

	for middle != secret {
		fmt.Printf("Guess a number between 1 and %d: ", high)
		scanner.Scan()

		guess, _ := strconv.ParseInt(scanner.Text(), 10, 64)
		middle = guess

		fmt.Println()
		if secret < guess {
			fmt.Println("Too high!")
		} else if secret > guess {
			fmt.Println("Too low!")
		}

		count++

	}
	fmt.Printf("Good Job! The number was %d!\n", secret)
	fmt.Printf("It took you %d tries.\n", count)
}

func main() {
	// clear screen
	fmt.Print("\033[H\033[2J")

	// array of games
	games := []Game{
		Game{"Guessing Game", guessing_game},
		Game{"Hangman", hangman},
	}

	// create scanner
	scanner := bufio.NewScanner(os.Stdin)

	fmt.Println("Hello! I am AMY, your terminal companion!")
	fmt.Print("What is your name? ")

	// get name
	scanner.Scan()
	name := scanner.Text()
	fmt.Printf("\nHi %s!\n", name)

	// game prompt
	fmt.Print("Would you like to play a game? ")
	scanner.Scan()
	fmt.Println()
	ans := scanner.Text()

	if ans == "" {
		fmt.Println("That's too bad. See you later!")
	} else if ans[0] == 89 || ans[0] == 121 {
		fmt.Println("Great! Which game do you want to play?")

		for i, ele := range games {
			fmt.Printf("%d. %s\n", i+1, ele.name)
		}

		fmt.Print("> ")
		scanner.Scan()
		choice, _ := strconv.ParseInt(scanner.Text(), 10, 64)

		fmt.Println()

		if choice > 0 && int(choice)-1 < len(games) {
			games[int(choice)-1].function()
		} else {
			fmt.Println("That's not an option! Try again.")
		}

	} else {
		fmt.Println("That's too bad. See you later!")
	}
}
