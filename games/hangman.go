package games

import (
	"fmt"
	"strings"
	"unicode"
)

func createBlank(s string, guesses map[string]bool) string {
	var blanks string

	for _, i := range s {
		_, ok := guesses[string(unicode.ToLower(i))]
		if ok {
			blanks += string(i) + " "
		} else {
			if unicode.IsLetter(i) {
				blanks += "_ "
			} else {
				blanks += string(i) + " "
			}
		}
	}
	return blanks
}

func hasWon(s string, guesses map[string]bool) bool {
	for _, i := range s {
		if unicode.IsLetter(i) {
			_, ok := guesses[string(unicode.ToLower(i))]
			if !ok {
				return false
			}
		}
	}
	return true
}

func formatGuesses(guesses map[string]bool) string {
	str := "Guesses [ "
	alphabet := "abcdefghijklmnopqrstuvwxyz"
	for _, i := range alphabet {
		_, ok := guesses[string(i)]
		if ok {
			str += string(i) + " "
		}
	}
	return str + "]"
}

func formatHangman(lives int) string {
	var head string
	var torso string
	var legs string

	if lives <= 5 {
		head = " O "
	}

	if lives <= 4 {
		switch lives {
		case 4:
			torso = " | "
		case 3:
			torso = "/| "
		default:
			torso = "/|\\"
		}
	}

	if lives <= 1 {
		switch lives {
		case 1:
			legs = "/  "
		default:
			legs = "/ \\"
		}
	}

	return fmt.Sprintf("    |==|\n    | %s\n    | %s\n    | %s\n  __|__\n", head, torso, legs)

	// |---|
	// |   O
	// |  /|\
	// |  / \
	// |_____
}

func Hangman() Result {
	lives := 6
	guesses := map[string]bool{}
	_, _ = lives, guesses
	word := RandomWord()

	for lives > 0 && !hasWon(word, guesses) {
		blanks := createBlank(word, guesses)

		fmt.Println(formatGuesses(guesses))
		fmt.Println()
		fmt.Printf(formatHangman(lives))
		fmt.Println(blanks)

		guess := GetUserString()

		fmt.Println()

		if len(guess) != 1 {
			fmt.Println("Guess a letter!")
			fmt.Println()
			continue
		} else if !unicode.IsLetter(rune(guess[0])) {
			fmt.Println("Guess a letter!")
			fmt.Println()
			continue
		}

		lowerGuess := string(unicode.ToLower(rune(guess[0])))
		_, ok := guesses[lowerGuess]

		if ok {
			fmt.Printf("You already guessed %s", lowerGuess)
			fmt.Println()
			fmt.Println()
			continue
		}

		guesses[lowerGuess] = true

		if strings.Contains(word, lowerGuess) {
			fmt.Printf("%s is in the word.", lowerGuess)
		} else {
			fmt.Printf("%s is not in the word.", lowerGuess)
			lives--
		}
		fmt.Println()
		fmt.Println()
	}

	fmt.Println(formatHangman(lives))
	fmt.Printf("The word was: %s\n", word)
	if hasWon(word, guesses) {
		fmt.Println("You Win!")
		return Result{
			Points:    8,
			PlayerWin: true,
		}
	} else if lives <= 0 {
		fmt.Println("You Lose...")
		return Result{
			Points:    1,
			PlayerWin: false,
		}
	}
	return Result{}
}
