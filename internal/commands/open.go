package commands

import (
	"context"
	"fmt"
	"glitter/internal/git"
	"glitter/internal/shell"
	"strings"

	"github.com/urfave/cli/v3"
)

func getProjectUrl() string {
	return strings.ReplaceAll(strings.TrimSpace(git.Origin()), ".git", "")
}

func newOpenCommand() *cli.Command {
	return &cli.Command{
		Name:      "open",
		Usage:     "Open the repository in the default web browser",
		ArgsUsage: "[commit]",
		Arguments: []cli.Argument{
			&cli.StringArg{
				Name:      "commit",
				UsageText: "Open a specific commit",
			},
		},
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:    "dump",
				Aliases: []string{"d"},
				Usage:   "Print the URL instead of opening it",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			url := getProjectUrl()
			if c.Bool("dump") {
				fmt.Println(url)
				return nil
			}
			return shell.Open(url)
		},
	}
}
