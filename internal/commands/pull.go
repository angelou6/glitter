package commands

import (
	"context"
	"glitter/internal/git"

	"github.com/manifoldco/promptui"
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
				prompt := promptui.Prompt{
					Label:     "This will wipe uncommited changes. Are you sure",
					IsConfirm: true,
				}

				_, err := prompt.Run()
				// selecting No is for some reason an error
				if err != nil {
					return err
				}
			}
			return git.ForcePull()
		},
	}
}
