package commands

import (
	"context"
	"fmt"
	"glitter/internal/git"

	"github.com/manifoldco/promptui"
	"github.com/urfave/cli/v3"
)

func selectFilesToToggle(files []git.File) []git.File {
	const doneLabel = "done"

	marked := make(map[int]bool, len(files))
	for i, f := range files {
		marked[i] = f.IsTracked
	}
	cursor := 0

	for {
		items := make([]string, 0, len(files)+1)
		for i, f := range files {
			checkbox := "[ ]"
			if marked[i] {
				checkbox = "[x]"
			}
			items = append(items, fmt.Sprintf("%s %s", checkbox, f.Path))
		}
		items = append(items, doneLabel)

		prompt := promptui.Select{
			Label:        "Toggle tracked state (enter to toggle, select done to confirm)",
			Items:        items,
			Size:         len(items),
			HideSelected: true,
			CursorPos:    cursor,
		}

		idx, result, err := prompt.Run()
		if err != nil || result == doneLabel {
			break
		}

		cursor = idx
		marked[idx] = !marked[idx]
	}

	var toggled []git.File
	for i, f := range files {
		if marked[i] != f.IsTracked {
			toggled = append(toggled, f)
		}
	}

	return toggled
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

			if len(files) == 0 {
				toggled := selectFilesToToggle(git.ParseStatus())
				for i := range toggled {
					toggled[i].Toggle()
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
