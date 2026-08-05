package replay

import (
	"context"
	"fmt"
	"glitter/internal/shell"
	"os"

	"github.com/urfave/cli/v3"
)

func newArgsCommand() *cli.Command {
	return &cli.Command{
		Name:      "show",
		Usage:     "Show the arguments of a replay",
		ArgsUsage: "<action>",
		Arguments: []cli.Argument{
			&cli.StringArg{
				Name:      "name",
				UsageText: "Name of the replay",
			},
		},
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:    "open",
				Aliases: []string{"o"},
				Usage:   "Open a replay in the default text editor",
			},
		},
		ShellComplete: fileCompletion,
		Action: func(ctx context.Context, c *cli.Command) error {
			replay := c.StringArg("name")
			if replay == "" {
				return cli.ShowSubcommandHelp(c)
			}

			open := c.Bool("open")
			dir, err := getFullDir(replay)
			if err != nil {
				return err
			}
			if open {
				return shell.Open(dir)
			} else {
				data, err := os.ReadFile(dir)
				if err != nil {
					return fmt.Errorf("Replay \"%s\" not found", replay)
				}
				fmt.Println(string(data))
			}

			return nil
		},
	}
}
