package commands

import (
	"context"
	"errors"
	"glitter/internal/git"
	"glitter/internal/shell"
	"os"
	"path/filepath"
	"strings"

	"github.com/manifoldco/promptui"
	"github.com/urfave/cli/v3"
)

func setupOrigin(remote string) {
	shell.Command("git", "remote", "add", "origin", remote)
}

func pushToOrigin() {
	branch, _ := shell.Command("git", "branch", "--show-current").Output()
	shell.Command("git", "push", "-u", "origin", strings.TrimSpace(branch)).Run()
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
				setupOrigin(origin)
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

			namePrompt := promptui.Prompt{
				Label: "Name",
				Validate: func(s string) error {
					if strings.ContainsAny(s, " ") {
						return errors.New("Name cannot contain spaces")
					}
					return nil
				},
				Default: cwd(),
			}
			name, err := namePrompt.Run()
			if err != nil {
				return err
			}

			descPrompt := promptui.Prompt{
				Label: "Description",
			}
			desc, err := descPrompt.Run()
			if err != nil {
				return err
			}

			selectPrompt := promptui.Select{
				Label: "Visibility",
				Items: []string{"private", "public"},
			}
			_, visibility, err := selectPrompt.Run()
			if err != nil {
				return err
			}

			return github(name, desc, visibility == "private")
		},
	}
}
