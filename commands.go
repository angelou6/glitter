package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func addAndCommit(message string) error {
	if err := exec.Command("git", "add", ".").Run(); err != nil {
		return err
	}

	err := exec.Command("git", "commit", "-m", message).Run()
	if err != nil {
		return err
	}
	return nil
}

func amendCommit() error {
	if err := exec.Command("git", "add", ".").Run(); err != nil {
		return err
	}

	err := exec.Command("git", "commit", "--amend", "--no-edit").Run()
	if err != nil {
		return err
	}

	return nil
}

func ForcePush(message, blame string) error {
	err := addAndCommit(message)
	if err != nil {
		return err
	}

	if blame != "" {
		err = exec.Command("git", "commit", "--amend", "--author", blame, "--no-edit").Run()
		if err != nil {
			return err
		}
	}

	err = exec.Command("git", "push", "--force").Run()
	if err != nil {
		return err
	}

	return nil
}

func ForcePull(skip bool) error {
	if !skip {
		reader := bufio.NewReader(os.Stdin)
		fmt.Print("This will wipe all unsaved changes. Are you sure? [y/N] ")
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

func PushAsLast() error {
	err := amendCommit()
	if err != nil {
		return err
	}

	err = exec.Command("git", "push", "--force-with-lease").Run()
	if err != nil {
		return err
	}

	return nil
}
