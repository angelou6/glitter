package commands

import (
	"context"
	"errors"
	"fmt"

	"glitter/internal/bubbles/multiselect"
	"glitter/internal/git"

	"github.com/urfave/cli/v3"
)

func newStageCommand() *cli.Command {
	return &cli.Command{
		Name:      "stage",
		Usage:     "Stage or unstage files",
		ArgsUsage: "[files]",
		Aliases:   []string{"add"},
		Arguments: []cli.Argument{
			&cli.StringArgs{
				Name:      "files",
				UsageText: "Files to be staged",
				Max:       -1,
			},
		},
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:    "unstage",
				Aliases: []string{"u"},
				Usage:   "Revert staged files",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			files := c.StringArgs("files")
			revert := c.Bool("unstage")

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
		ShellComplete: func(ctx context.Context, c *cli.Command) {
			revert := c.Bool("unstage")
			files := git.ParseStatus()
			for _, f := range files {
				if f.IsTracked == revert {
					fmt.Println(f.Path)
				}
			}
		},
	}
}
