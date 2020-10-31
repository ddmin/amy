package games

import (
	"fmt"
)

const (
	easy   = 10
	medium = 100
	hard   = 1000
	insane = 1000000
)

type difficulty struct {
	n     string
	high  int
	lives int
}

func chooseDifficulty() *difficulty {
	difficulties := [...]difficulty{
		{"EASY", easy, 5},
		{"MEDIUM", medium, 10},
		{"HARD", hard, 20},
		{"INSANE", insane, 50},
	}

	for i, d := range difficulties {
		fmt.Printf("%d. %s\n", i+1, d.n)
	}
	input := GetUserInt()
	if input < 1 || input > 4 {
		input = 1
	}
	choice := difficulties[input-1]
	return &choice
}

func GuessingGame() {
	fmt.Println("Choose a difficulty:")
	gameDifficulty := chooseDifficulty()
	actual := PseudoRandom(1, gameDifficulty.high)

	fmt.Printf("\nGuess my number! (1-%d)\n", gameDifficulty.high)

	userGuess := 0
	lives := gameDifficulty.lives

	for lives > 0 {
		fmt.Printf("Lives: %d\n", lives)
		userGuess = GetUserInt()
		if userGuess < actual {
			fmt.Println("Too Low!")
			lives--
		} else if userGuess > actual {
			fmt.Println("Too High!")
			lives--
		} else {
			break
		}
		fmt.Println()
	}

	if userGuess == actual {
		fmt.Println()
		fmt.Println("You Win!")
	} else {
		fmt.Println("You Lose!")
		fmt.Printf("The number was: %d\n", actual)
	}
}
