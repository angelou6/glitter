package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func runCommand(name string, args ...string) error {
	cmd := exec.Command(name, args...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

func addAndCommit(message string, force bool) error {
	if force && message == "" {
		message = "i don't know"
	}

	if err := runCommand("git", "add", "."); err != nil {
		return err
	}

	err := runCommand("git", "commit", "-m", message)
	if err != nil {
		return err
	}
	return nil
}

func amendCommit(message string) error {
	if err := runCommand("git", "add", "."); err != nil {
		return err
	}

	if message == "" {
		err := runCommand("git", "commit", "--amend", "--no-edit")
		if err != nil {
			return err
		}
	} else {
		err := runCommand("git", "commit", "--amend", "-m", message)
		if err != nil {
			return err
		}
	}

	return nil
}

func ForcePush(message, blame string, force bool) error {
	err := addAndCommit(message, force)
	if err != nil {
		return err
	}

	if blame != "" {
		err = runCommand("git", "commit", "--amend", "--author", blame, "--no-edit")
		if err != nil {
			return err
		}
	}

	if force {
		err = runCommand("git", "push", "--force")
		if err != nil {
			return err
		}
	} else {
		err = runCommand("git", "push")
		if err != nil {
			return err
		}
	}

	return nil
}

func ForcePull(skip bool) error {
	if !skip {
		reader := bufio.NewReader(os.Stdin)
		fmt.Print("This will wipe all uncommited changes. Are you sure? [y/N] ")
		input, _ := reader.ReadString('\n')
		sure := strings.TrimSpace(input)

		if sure == "" {
			sure = "n"
		}

		lowerdSure := strings.ToLower(sure)

		if lowerdSure != "n" && lowerdSure != "y" {
			return errors.New("That is not an option.")
		}

		if lowerdSure == "n" {
			return nil
		}
	}

	if err := runCommand("git", "fetch", "--all"); err != nil {
		return err
	}
	if err := runCommand("git", "reset", "--hard"); err != nil {
		return err
	}
	return nil
}

func PushAsLast(message string, force bool) error {
	err := amendCommit(message)
	if err != nil {
		return err
	}

	f := "--force-with-lease"
	if force {
		f = "--force"
	}

	err = runCommand("git", "push", f)
	if err != nil {
		return err
	}

	return nil
}
