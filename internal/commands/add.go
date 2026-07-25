package commands

import (
	"context"
	"errors"
	"glitter/internal/bubbles/multiselect"
	"glitter/internal/git"

	"github.com/urfave/cli/v3"
)

func newAddCommand() *cli.Command {
	return &cli.Command{
		Name:      "add",
		Usage:     "Stage or unstage files",
		ArgsUsage: "[files]",
		Arguments: []cli.Argument{
			&cli.StringArgs{
				Name:      "files",
				UsageText: "Files to be staged",
				Max:       -1,
			},
		},
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:    "revert",
				Aliases: []string{"r"},
				Usage:   "Revert staged files",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			files := c.StringArgs("files")
			revert := c.Bool("revert")

			if len(files) == 0 {
				if !git.HasChanges() {
					return errors.New("There are no changes in this directory")
				}

				files := git.ParseStatus()
				elems := make([]multiselect.Element, len(files))
				for i := range files {
					elems[i] = &files[i]
				}

				return multiselect.New(elems).Run()
			}

			if revert {
				git.Unstage(files...)
			} else {
				git.Stage(files...)
			}
			return nil
		},
	}
}
