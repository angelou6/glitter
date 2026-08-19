package commands

import (
	"context"
	"glitter/internal/git"
	"glitter/internal/shell"

	"github.com/urfave/cli/v3"
)

func amendPush(messages []string, force, all bool) error {
	if err := amendCommit(messages, all); err != nil {
		return err
	}

	if force {
		return shell.Command("git", "push", "--force").Run()
	}
	return shell.Command("git", "push", "--force-with-lease").Run()
}

func newPushCommand() *cli.Command {
	return &cli.Command{
		Name:  "push",
		Usage: "Stage, commit, and push changes",
		Flags: []cli.Flag{
			&cli.StringSliceFlag{
				Name:    "message",
				Usage:   "Commit messages",
				Aliases: []string{"m"},
			},
			&cli.BoolFlag{
				Name:  "ammend",
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
			messages := c.StringSlice("message")
			amend := c.Bool("amend")
			all := c.Bool("all")
			force := c.Bool("force")

			if amend {
				return amendPush(messages, force, all)
			}
			return git.Push(messages, force, all)
		},
	}
}
