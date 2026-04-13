package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func addAndCommit(message string, force bool) error {
	if err := exec.Command("git", "add", ".").Run(); err != nil {
		return err
	}

	if !force {
		err := exec.Command("git", "commit", "-m", message).Run()
		if err != nil {
			return err
		}
	} else {
		err := exec.Command("git", "commit", "-m", "I don't know").Run()
		if err != nil {
			return err
		}
	}
	return nil
}

func amendCommit(message string) error {
	if err := exec.Command("git", "add", ".").Run(); err != nil {
		return err
	}

	if message == "" {
		err := exec.Command("git", "commit", "--amend", "--no-edit").Run()
		if err != nil {
			return err
		}
	} else {
		err := exec.Command("git", "commit", "--amend", "-m", message).Run()
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
		err = exec.Command("git", "commit", "--amend", "--author", blame, "--no-edit").Run()
		if err != nil {
			return err
		}
	}

	if force {
		err = exec.Command("git", "push", "--force").Run()
		if err != nil {
			return err
		}
	} else {
		err = exec.Command("git", "push").Run()
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

	if err := exec.Command("git", "fetch", "--all").Run(); err != nil {
		return err
	}
	if err := exec.Command("git", "reset", "--hard").Run(); err != nil {
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

	err = exec.Command("git", "push", f).Run()
	if err != nil {
		return err
	}

	return nil
}
