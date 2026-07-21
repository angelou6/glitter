package commands

import (
	"context"
	"errors"
	"glitter/internal/git"
	"glitter/internal/shell"

	"github.com/urfave/cli/v3"
)

func initCommand(messages []string, branch string) {
	shell.Command("git", "init").Run()
	shell.Command("git", "branch", "-M", branch).Run()
	if len(messages) == 0 {
		messages = []string{"Initial commit"}
	}
	git.StageAndCommit(messages, false, true)
}

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
			initCommand(messages, branch)

			return nil
		},
	}
}
