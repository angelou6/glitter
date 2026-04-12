package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	pushCmd := flag.NewFlagSet("push", flag.ExitOnError)
	last := pushCmd.Bool("last", false, "Amend all new modifications to the latest push.")
	blame := pushCmd.String("blame", "", "Blame this person for the commit (Author <email>).")
	message := pushCmd.String("m", "fuck you", "Commit message.")

	pullCmd := flag.NewFlagSet("pull", flag.ExitOnError)
	skip := pullCmd.Bool("fu", false, "Fuck you. Skip all warnings.")

	usage := func() {
		fmt.Println("Usage: gitfuckyou <command> [arguments]")
		fmt.Println("\nCommands:")
		fmt.Println("  push    Force push changes with an optional blame")
		fmt.Println("  pull    Force pull and reset local changes")
		fmt.Println("\nUse 'gitfuckyou <command> -h' for more information about a command.")
	}

	if len(os.Args) < 2 {
		usage()
		os.Exit(1)
	}

	switch os.Args[1] {
	case "push":
		pushCmd.Parse(os.Args[2:])
		if *last {
			PushAsLast()
		} else {
			ForcePush(*message, *blame)
		}
	case "pull":
		pullCmd.Parse(os.Args[2:])
		ForcePull(*skip)
	case "-h", "--help", "help":
		usage()
	default:
		fmt.Printf("Unknown command: %s\n", os.Args[1])
		usage()
		os.Exit(1)
	}
}
