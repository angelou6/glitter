package replay

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/urfave/cli/v3"
)

func buildString(args []string, usegit bool) string {
	var builder strings.Builder

	if usegit {
		builder.WriteString("//usegit//\n")
	}
	builder.WriteString(strings.Join(args, "\n"))

	return builder.String()
}

func writeFile(name string, args []string, usegit bool) error {
	confDir, err := getConfigDir()
	if err != nil {
		return err
	}

	f, err := os.Create(filepath.Join(confDir, name))
	if err != nil {
		return err
	}
	defer f.Close()

	f.WriteString(buildString(args, usegit))

	return nil
}

func newCreateCommand() *cli.Command {
	return &cli.Command{
		Name:      "create",
		Usage:     "Create a new replay or modify an existing one",
		ArgsUsage: "<action> [commit]",
		Arguments: []cli.Argument{
			&cli.StringArg{
				Name:      "name",
				UsageText: "Name of the replay",
			},
			&cli.StringArgs{
				Name:      "args",
				UsageText: "The arguments for the program",
				Min:       1,
				Max:       -1,
			},
		},
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:  "git",
				Usage: "Use git instead of glitter",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			name := c.StringArg("name")
			args := c.StringArgs("args")
			useGit := c.Bool("git")

			if err := writeFile(name, args, useGit); err != nil {
				return err
			}

			fmt.Println("Replay created successfully")
			return nil
		},
	}
}
