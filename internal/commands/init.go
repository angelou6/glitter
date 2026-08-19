package commands

import (
	"context"
	"errors"

	"glitter/internal/git"

	"github.com/urfave/cli/v3"
)

func newInitCommand() *cli.Command {
	return &cli.Command{
		Name:  "init",
		Usage: "Initialize a git repo",
		Flags: []cli.Flag{
			&cli.StringSliceFlag{
				Name:    "message",
				Aliases: []string{"m"},
				Usage:   "Commit messages",
			},
			&cli.StringFlag{
				Name:  "branch",
				Usage: "Declare branch",
				Value: "main",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			if git.IsRepo() {
				return errors.New("This directory has already been initialized")
			}
			messages := c.StringSlice("message")
			branch := c.String("branch")
			git.InitCommand(messages, branch)

			return nil
		},
	}
}
