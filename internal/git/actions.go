package git

import (
	"fmt"
	"glitter/internal/shell"
	"strings"
)

func Pull() error {
	return shell.Command("git", "pull").Run()
}

func StageAndCommit(messages []string, all bool) error {
	if len(StagedFiles()) == 0 || all {
		Stage(".")
	}

	args := []string{"commit"}
	staged := StagedFiles()

	if len(messages) == 0 {
		stagedFiles := len(staged)
		plural := ""
		if stagedFiles != 1 {
			plural = "s"
		}

		args = append(args, MessagesToArgs([]string{
			fmt.Sprintf("Changed %d file%s", stagedFiles, plural),
			fmt.Sprintf("-m Files changed: %s", strings.Join(staged, ", ")),
		})...)
		return shell.Command("git", args...).Run()
	}

	args = append(args, MessagesToArgs(messages)...)
	return shell.Command("git", args...).Run()
}

func Push(messages []string, force, all bool) error {
	if !hasUnpushedCommits() {
		err := StageAndCommit(messages, all)
		if err != nil {
			return err
		}
	}

	if force {
		return shell.Command("git", "push", "--force").Run()
	}
	return shell.Command("git", "push").Run()
}
