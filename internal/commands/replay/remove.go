package replay

import (
	"context"
	"fmt"
	"os"

	"github.com/urfave/cli/v3"
)

func newRemoveCommand() *cli.Command {
	return &cli.Command{
		Name:  "remove",
		Usage: "Remove replay",
		Arguments: []cli.Argument{
			&cli.StringArg{
				Name:      "replay",
				UsageText: "Name of the replay to play",
			},
		},
		ShellComplete: fileCompletion,
		Action: func(ctx context.Context, c *cli.Command) error {
			replay := c.StringArg("replay")
			dir, err := getFullDir(replay)
			if err != nil {
				return err
			}

			if err := os.Remove(dir); err != nil {
				return err
			}
			fmt.Println("Deleted successfully")

			return nil
		},
	}
}
