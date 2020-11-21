package games

import (
	"fmt"
)

func display(amyNums []int, playerNums []int) {
	amy := "AMY: "
	player := "YOU: "

	for _, i := range amyNums {
		if i != 0 {
			amy += fmt.Sprintf("%2v ", i)
		} else {
			amy += "   "
		}
	}

	for _, i := range playerNums {
		if i != 0 {
			player += fmt.Sprintf("%2v ", i)
		} else {
			player += "   "
		}
	}

	fmt.Println()
	fmt.Println(amy)
	fmt.Println(player)
	fmt.Println()
}

// update slice to contain elements up to integer n (taking into account c, the quantity of numbers said)
func ListTo(arr []int, n int, c int) []int {
	if len(arr) == 0 {
		for i := 1; i <= n-c; i++ {
			arr = append(arr, 0)
		}
		for i := n - c + 1; i <= n; i++ {
			arr = append(arr, i)
		}
	} else {
		for i := arr[len(arr)-1] + 1; i <= n-c; i++ {
			arr = append(arr, 0)
		}
		for i := n - c + 1; i <= n; i++ {
			arr = append(arr, i)
		}
	}
	return arr
}

func Poison() Result {
	players := []string{"AMY", "YOU"}
	currNum := 0
	currPlayer := PseudoRandom(0, 1)

	amyNums := []int{}
	playerNums := []int{}

	_, _, _ = amyNums, playerNums, currNum

	for currNum < 21 {
		fmt.Printf("CURRENT PLAYER: %s\n", players[currPlayer])
		if players[currPlayer] == "YOU" {
			n := 0
			for n != 1 && n != 2 {
				fmt.Println("How many number to say? [1, 2]")
				n = GetUserInt()
				if n != 1 && n != 2 {
					fmt.Println("Try again.")
				}
			}
			currNum += n
			playerNums = ListTo(playerNums, currNum, n)
			display(amyNums, playerNums)
		} else {
			n := 0
			if currNum == 18 {
				n = 2
			} else if currNum == 19 || currNum == 20 {
				n = 1
			} else {
				n = PseudoRandom(1, 2)
			}
			fmt.Printf("AMY said %v number(s).\n", n)
			currNum += n
			amyNums = ListTo(amyNums, currNum, n)
			display(amyNums, playerNums)
		}

		if currNum >= 21 {
			switch players[currPlayer] {
			case "YOU":
				fmt.Println("You said '21'!")
				fmt.Println("You Lose!")
				return Result{
					Points:    1,
					PlayerWin: false,
				}
			case "AMY":
				fmt.Println("AMY said '21'!")
				fmt.Println("You Win!")
				return Result{
					Points:    1,
					PlayerWin: true,
				}
			}
		}

		// toggle player
		currPlayer = (currPlayer + 1) % 2
	}
	return Result{}
}
