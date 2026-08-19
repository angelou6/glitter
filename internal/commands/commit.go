package commands

import (
	"context"
	"errors"
	"glitter/internal/git"
	"glitter/internal/shell"

	"github.com/urfave/cli/v3"
)

func amendCommit(messages []string, all bool) error {
	if len(git.StagedFiles()) == 0 || all {
		git.Stage(".")
	}

	if len(messages) == 0 {
		return shell.Command("git", "commit", "--amend", "--no-edit").Run()
	}
	args := []string{"git", "commit", "--amend"}
	args = append(args, git.MessagesToArgs(messages)...)
	return shell.Command(args...).Run()
}

func newCommitCommand() *cli.Command {
	return &cli.Command{
		Name:  "commit",
		Usage: "Stage all files and commit",
		Flags: []cli.Flag{
			&cli.StringSliceFlag{
				Name:    "message",
				Usage:   "Commit messages",
				Aliases: []string{"m"},
			},
			&cli.BoolFlag{
				Name:  "amend",
				Usage: "Amend all new modifications to latest",
			},
			&cli.BoolFlag{
				Name:    "force",
				Usage:   "Force command to execute",
				Aliases: []string{"f"},
			},
			&cli.BoolFlag{
				Name:    "all",
				Usage:   "Ignore staged files and stage all",
				Aliases: []string{"a"},
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			if !git.HasChanges() {
				return errors.New("There are no changes to commit")
			}

			messages := c.StringSlice("message")
			amend := c.Bool("amend")
			all := c.Bool("all")
			if amend {
				return amendCommit(messages, all)
			}

			force := c.Bool("force")
			return git.StageAndCommit(messages, force, all)
		},
	}
}
