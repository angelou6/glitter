package replay

import (
	"context"
	"fmt"

	"github.com/urfave/cli/v3"
)

func newListCommand() *cli.Command {
	return &cli.Command{
		Name:          "list",
		Usage:         "list available replays",
		ShellComplete: fileCompletion,
		Action: func(ctx context.Context, c *cli.Command) error {

			entries, err := getReplayEntries()
			if err != nil {
				return err
			}

			for _, entry := range entries {
				fmt.Println(entry)
			}

			return nil
		},
	}
}
