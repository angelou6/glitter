package commands

import (
	"context"
	"errors"
	"fmt"
	"glitter/internal/git"
	"glitter/internal/shell"

	"github.com/urfave/cli/v3"
)

func undoCommit(hard bool, commit string) error {
	if !git.RepoHasCommits() {
		return errors.New("This repo has no commits")
	}

	if hard {
		return shell.Command("git", "reset", "--hard", commit).Run()
	}
	return shell.Command("git", "reset", commit).Run()
}

func undoPush(hard bool, commit string) error {
	if err := undoCommit(hard, commit); err != nil {
		return err
	}
	return shell.Command("git", "push", "--force-with-lease").Run()
}

func newUndoCommand() *cli.Command {
	return &cli.Command{
		Name:      "undo",
		Usage:     "Undo an action",
		ArgsUsage: "<action> [commit]",
		Arguments: []cli.Argument{
			&cli.StringArg{
				Name:      "action",
				UsageText: "Target of undo command [possible values: commit, push]",
			},
			&cli.StringArg{
				Name:      "commit",
				Value:     "HEAD~",
				UsageText: "Revert to this commit",
			},
		},
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:  "hard",
				Usage: "Also undo changes locally",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			action := c.StringArg("action")
			commit := c.StringArg("commit")
			hard := c.Bool("hard")

			switch action {
			case "commit":
				return undoCommit(hard, commit)
			case "push":
				return undoPush(hard, commit)
			}
			return fmt.Errorf("Action '%s' is invalid", action)
		},
	}
}
