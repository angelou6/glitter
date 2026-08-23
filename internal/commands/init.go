package commands

import (
	"context"
	"errors"

	"github.com/urfave/cli/v3"
	"glitter/internal/git"
)

func newInitCommand() *cli.Command {
	return &cli.Command{
		Name:  "init",
		Usage: "Initialize a git repo and commit",
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:    "no-commit",
				Aliases: []string{"n"},
				Usage:   "Initialize the repo without commiting",
			},
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
			noCommit := c.Bool("no-commit")
			branch := c.String("branch")
			messages := c.StringSlice("message")

			git.Initialize(branch)

			if !noCommit {
				if len(messages) == 0 {
					messages = []string{"Initial commit"}
				}
				return git.StageAndCommit(messages, true)
			}
			return nil
		},
	}
}
