package commands

import (
	"context"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"glitter/internal/bubbles/confirm"
	"glitter/internal/bubbles/input"
	"glitter/internal/bubbles/options"
	"glitter/internal/git"
	"glitter/internal/platform"
	"glitter/internal/shell"

	"github.com/urfave/cli/v3"
)

func pushToOrigin() error {
	branch, _ := shell.Command("git", "branch", "--show-current").Output()
	return shell.Command("git", "push", "-u", "origin", strings.TrimSpace(branch)).Run()
}

func cwd() string {
	dir, err := os.Getwd()
	if err != nil {
		panic(err)
	}

	return filepath.Base(dir)
}

func publishCommand() *cli.Command {
	return &cli.Command{
		Name:  "publish",
		Usage: "Publish to github or origin",
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:    "codeberg",
				Aliases: []string{"c"},
			},
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

			useCodeberg := c.Bool("codeberg")
			if err := platform.CheckExistance(useCodeberg); err != nil {
				return err
			}

			if c.IsSet("name") || c.IsSet("desc") || c.IsSet("private") {
				name := c.String("name")
				desc := c.String("desc")
				private := c.Bool("private")

				if len(name) == 0 {
					return errors.New("Name needs to be given for this command")
				}

				if useCodeberg {
					return platform.Publish(platform.Codeberg, name, desc, private)
				}
				return platform.Publish(platform.GitHub, name, desc, private)
			}

			publishPlatform, err := options.New("Publish platform", []string{"github", "codeberg"}).Run()
			if err != nil {
				return err
			}

			name, err := input.New(
				"Name", cwd(),
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

			if publishPlatform == "codeberg" {
				return platform.Publish(platform.Codeberg, name, desc, visibility == "private")
			}
			return platform.Publish(platform.GitHub, name, desc, visibility == "private")
		},
	}
}
