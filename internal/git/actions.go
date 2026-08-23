package git

import "glitter/internal/shell"

func Pull() error {
	return shell.Command("git", "pull").Run()
}

func StageAndCommit(messages []string, all bool) error {
	if len(StagedFiles()) == 0 || all {
		Stage(".")
	}

	args := []string{"commit"}

	if len(messages) == 0 {
		return shell.Command("git", AutoGenerateMessage()...).Run()
	}

	args = append(args, MessagesToArgs(messages)...)
	return shell.Command("git", args...).Run()
}

func Push(messages []string, force, all bool) error {
	if !HasUnpushedCommits() || len(messages) > 0 {
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

func Initialize(branch string) error {
	err := shell.Command("git", "init").Run()
	if err != nil {
		return err
	}

	err = shell.Command("git", "branch", "-M", branch).Run()
	if err != nil {
		return err
	}

	return nil
}
