package commands

import (
	"context"
	"errors"
	"fmt"
	"glitter/internal/shell"
	"runtime"
	"strings"

	"github.com/urfave/cli/v3"
)

func open(url string) error {
	switch runtime.GOOS {
	case "linux":
		return shell.Command("xdg-open", url).Spawn()
	case "windows":
		return shell.Command("cmd", "/c", "start", url).Spawn()
	case "darwin":
		return shell.Command("open", url).Spawn()
	}
	return errors.New("OS not found")
}

func getProjectUrl() string {
	remote, _ := shell.Command("git", "remote", "get-url", "origin").Output()
	remote = strings.ReplaceAll(strings.TrimSpace(remote), ".git", "")
	return remote
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
			return open(url)
		},
	}
}
