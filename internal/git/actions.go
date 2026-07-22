package git

import (
	"fmt"
	"glitter/internal/shell"
	"strings"
)

func ForcePull() error {
	if err := shell.Command("git", "fetch", "origin").Run(); err != nil {
		return err
	}
	return shell.Command("git", "reset", "--hard", "@{u}").Run()
}

func Pull() error {
	return shell.Command("git", "pull").Run()
}

func StageAndCommit(messages []string, force, all bool) error {
	if len(StagedFiles()) == 0 || all {
		Stage(".")
	}

	args := []string{"git", "commit"}
	staged := StagedFiles()

	if len(messages) == 0 && force {
		stagedFiles := len(staged)
		plural := ""
		if stagedFiles > 0 {
			plural = "s"
		}

		args = append(args, MessagesToArgs([]string{
			fmt.Sprintf("Changed %d file%s", stagedFiles, plural),
			fmt.Sprintf("-m Files changed: %s", strings.Join(staged, ", ")),
		})...)
		return shell.Command(args...).Run()
	}

	args = append(args, MessagesToArgs(messages)...)
	return shell.Command(args...).Run()
}

func Push(messages []string, force, all bool) error {
	if err := StageAndCommit(messages, force, all); err != nil {
		return err
	}

	if force {
		return shell.Command("git", "push", "--force").Run()
	}
	return shell.Command("git", "push").Run()
}
