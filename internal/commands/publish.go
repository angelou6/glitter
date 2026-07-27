package commands

import (
	"context"
	"errors"
	"fmt"
	"glitter/internal/bubbles/confirm"
	"glitter/internal/bubbles/input"
	"glitter/internal/bubbles/options"
	"glitter/internal/git"
	"glitter/internal/shell"
	"os"
	"path/filepath"
	"strings"

	"github.com/urfave/cli/v3"
)

func pushToOrigin() error {
	branch, _ := shell.Command("git", "branch", "--show-current").Output()
	return shell.Command("git", "push", "-u", "origin", strings.TrimSpace(branch)).Run()
}

func github(name, desc string, private bool) error {
	args := []string{"gh", "repo", "create", name, "--description", desc, "--source", ".", "--remote=origin", "--push"}
	if private {
		args = append(args, "--private")
	} else {
		args = append(args, "--public")
	}

	return shell.Command(args...).Run()
}

func cwd() string {
	dir, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	return filepath.Base(dir)
}

func newPublishCommand() *cli.Command {
	return &cli.Command{
		Name:  "publish",
		Usage: "Publish to github or origin",
		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:    "name",
				Aliases: []string{"n"},
			},
			&cli.StringFlag{
				Name:    "desc",
				Aliases: []string{"d"},
			},
			&cli.StringFlag{
				Name:    "private",
				Aliases: []string{"p"},
			},
			&cli.StringFlag{
				Name:    "origin",
				Aliases: []string{"o"},
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			if !git.IsRepo() {
				return errors.New("This is not a repo")
			}

			origin := c.String("origin")
			// TODO: Error handling here
			if origin != "" {
				originalOrigin, _ := shell.Command("git", "config", "--get", "remote.origin.url").Output()
				if originalOrigin != "" {
					fmt.Printf("An origin already exists (%s)\n", originalOrigin)
					res, _ := confirm.Run(fmt.Sprintf("Do you want to replace it with %s?", origin))
					if !res {
						return nil
					}
					shell.Command("git", "remote", "remove", "origin").Run()
				}
				shell.Command("git", "remote", "add", "origin", origin).Run()
				pushToOrigin()

				return nil
			}

			if !shell.Exists("gh") {
				return errors.New("github-cli not found")
			}

			if c.IsSet("name") || c.IsSet("desc") || c.IsSet("private") {
				name := c.String("name")
				desc := c.String("desc")
				private := c.Bool("private")

				if len(name) == 0 {
					return errors.New("Name needs to be given for this command")
				}

				return github(name, desc, private)
			}

			name, err := input.New("Name", cwd(),
				func(s string) error {
					if strings.Contains(s, " ") {
						return errors.New("Cannot contain spaces")
					}
					return nil
				},
			).Run()
			if err != nil {
				return err
			}

			desc, err := input.New("Description", "", func(s string) error { return nil }).Run()
			if err != nil {
				return err
			}

			visibility, err := options.New("Project visibility", []string{"private", "public"}).Run()
			if err != nil {
				return err
			}

			return github(name, desc, visibility == "private")
		},
	}
}
