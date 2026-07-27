package commands

import (
	"context"
	"glitter/internal/bubbles/confirm"
	"glitter/internal/git"
	"glitter/internal/shell"

	"github.com/urfave/cli/v3"
)

func ForcePull() error {
	if err := shell.Command("git", "fetch", "origin").Run(); err != nil {
		return err
	}
	if err := shell.Command("git", "reset", "--hard", "@{u}").Run(); err != nil {
		return err
	}
	return shell.Command("git", "clean", "-fd").Run()
}

func newPullCommand() *cli.Command {
	return &cli.Command{
		Name:  "pull",
		Usage: "Pull changes from remote",
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:    "force",
				Usage:   "Removes local changes and pulls from remote",
				Aliases: []string{"f"},
			},
			&cli.BoolFlag{
				Name:    "yes",
				Usage:   "Skip force pull warning",
				Aliases: []string{"y"},
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			skip := c.Bool("yes")
			force := c.Bool("force")

			if force {
				if !skip {
					res, _ := confirm.Run("This action will wipe all local changes, are you sure?")
					if !res {
						return nil
					}
				}

				return ForcePull()
			}

			return git.Pull()
		},
	}
}
