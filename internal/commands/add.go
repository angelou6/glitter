package commands

import (
	"context"
	"glitter/internal/git"

	"charm.land/huh/v2"
	"github.com/urfave/cli/v3"
)

func fileOptions() []huh.Option[git.File] {
	var options []huh.Option[git.File]
	files := git.ParseStatus()

	for _, f := range files {
		options = append(options, huh.NewOption(f.Path, f).Selected(f.IsTracked))
	}
	return options
}

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
			var selected []git.File

			if len(files) == 0 {
				f := fileOptions()

				huh.NewMultiSelect[git.File]().
					Title("Files").
					Options(f...).
					Value(&selected).
					Height(len(f) + 1).
					Run()

				for _, s := range selected {
					s.Toggle()
				}

				return nil
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
