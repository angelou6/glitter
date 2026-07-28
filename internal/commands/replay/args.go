package replay

import (
	"context"
	"fmt"
	"os"
	"path/filepath"

	"github.com/urfave/cli/v3"
)

func newArgsCommand() *cli.Command {
	return &cli.Command{
		Name:      "args",
		Usage:     "Show the arguments of a replay",
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
			data, err := os.ReadFile(filepath.Join(confDir, replay))
			if err != nil {
				return fmt.Errorf("Replay \"%s\" not found", replay)
			}
			fmt.Println(string(data))

			return nil
		},
	}
}
