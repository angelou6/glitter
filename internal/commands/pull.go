package commands

import (
	"bufio"
	"context"
	"glitter/internal/git"
	"os"
	"unicode"

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

			if force {
				if !skip {
					reader := bufio.NewReader(os.Stdin)
					char, _, _ := reader.ReadRune()
					if unicode.ToLower(char) == 'n' {
						return nil
					}
				}

				return git.ForcePull()
			}

			return git.Pull()
		},
	}
}
