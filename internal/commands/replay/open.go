package replay

import (
	"context"
	"errors"
	"os"

	"glitter/internal/shell"

	"github.com/urfave/cli/v3"
)

func newOpenCommand() *cli.Command {
	return &cli.Command{
		Name:          "open",
		Usage:         "open a replay",
		ArgsUsage:     "<action>",
		ShellComplete: fileCompletion,
		Action: func(ctx context.Context, c *cli.Command) error {
			var dir string
			var err error

			if c.Args().Len() > 0 {
				dir, err = getFullDir(c.Args().First())
				if _, err := os.Stat(dir); errors.Is(err, os.ErrNotExist) {
					return err
				}
			} else {
				dir, err = getConfigDir()
				if err != nil {
					return err
				}
			}

			shell.Open(dir)

			return nil
		},
	}
}
