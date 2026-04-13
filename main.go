package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	pushCmd := flag.NewFlagSet("push", flag.ExitOnError)
	last := pushCmd.Bool("last", false, "Amend all new modifications to the latest push.")
	force := pushCmd.Bool("force", false, "Force push.")
	blame := pushCmd.String("blame", "", "Blame this person for the commit (Author <email>).")
	message := pushCmd.String("m", "", "Commit message.")

	pullCmd := flag.NewFlagSet("pull", flag.ExitOnError)
	skip := pullCmd.Bool("y", false, "Skip warning.")

	usage := func() {
		fmt.Println("Usage: glitter <command> [arguments]")
		fmt.Println("\nCommands:")
		fmt.Println("  push    Force push changes with an optional blame")
		fmt.Println("  pull    Force pull and reset local changes")
		fmt.Println("\nUse 'glitter <command> -h' for more information about a command.")
	}

	if len(os.Args) < 2 {
		usage()
		os.Exit(1)
	}

	switch os.Args[1] {
	case "push":
		pushCmd.Parse(os.Args[2:])
		if *last {
			if *blame != "" {
				fmt.Println("You cannot use -blame with -last")
				os.Exit(1)
			}

			err := PushAsLast(*message, *force)
			if err != nil {
				fmt.Println(err)
			}
		} else {
			err := ForcePush(*message, *blame, *force)
			if err != nil {
				fmt.Println(err)
			}
		}
	case "pull":
		pullCmd.Parse(os.Args[2:])
		err := ForcePull(*skip)
		if err != nil {
			fmt.Println(err)
		}
	case "-h", "--help", "help":
		usage()
	default:
		fmt.Printf("Unknown command: %s\n", os.Args[1])
		usage()
		os.Exit(1)
	}
}
