package replay

import (
	"context"
	"glitter/internal/shell"
	"path/filepath"

	"github.com/urfave/cli/v3"
)

func newOpenCommand() *cli.Command {
	return &cli.Command{
		Name:      "open",
		Usage:     "Open a replay in the default text editor",
		ArgsUsage: "<action> [commit]",
		Arguments: []cli.Argument{
			&cli.StringArg{
				Name:      "name",
				UsageText: "Name of the replay",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			replay := c.StringArg("name")
			if replay == "" {
				return cli.ShowSubcommandHelp(c)
			}

			confDir, err := getConfigDir()
			if err != nil {
				return err
			}
			shell.Open(filepath.Join(confDir, replay))

			return nil
		},
	}
}
