package commands

import (
	"context"
	"glitter/internal/git"
	"os"

	"charm.land/huh/v2"
	"github.com/urfave/cli/v3"
)

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

			if !force {
				return git.Pull()
			}

			if !skip {
				var confirm bool
				huh.NewConfirm().
					Title("This will wipe uncommited changes. Are you sure?").
					Affirmative("Yes").
					Negative("No").
					Value(&confirm).
					Run()

				if !confirm {
					os.Exit(0)
				}
			}
			return git.ForcePull()
		},
	}
}
