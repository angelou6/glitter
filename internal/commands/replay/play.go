package replay

import (
	"context"
	"fmt"
	"glitter/internal/colorize"
	"glitter/internal/shell"
	"os"
	"strings"

	"github.com/urfave/cli/v3"
)

func getConfigEntries() ([]os.DirEntry, error) {
	confDir, err := getConfigDir()
	if err != nil {
		return []os.DirEntry{}, err
	}
	entries, err := os.ReadDir(confDir)
	if err != nil {
		return []os.DirEntry{}, err
	}

	return entries, nil
}

func parseReplay(replay string) (bool, []string, error) {
	dir, err := getFullDir(replay)
	if err != nil {
		return false, []string{}, err
	}
	data, err := os.ReadFile(dir)
	if err != nil {
		return false, []string{}, fmt.Errorf("Replay \"%s\" not found", replay)
	}

	args := strings.Split(string(data), "\n")
	if args[0] == "//usegit//" {
		return true, args[1:], nil
	}

	return false, args, nil
}

func newPlayCommand() *cli.Command {
	return &cli.Command{
		Name:      "play",
		Usage:     "Play replay",
		ArgsUsage: "<action> [commit]",
		Arguments: []cli.Argument{
			&cli.StringArg{
				Name:      "replay",
				UsageText: "Name of the replay to play",
			},
		},
		ShellComplete: fileCompletion,
		Action: func(ctx context.Context, c *cli.Command) error {
			replay := c.StringArg("replay")
			if replay == "" {
				entries, err := getConfigEntries()
				if err != nil {
					return err
				}

				if len(entries) > 0 {
					for _, e := range entries {
						fmt.Println(e)
					}
				} else {
					cli.ShowSubcommandHelp(c)
				}

				return nil
			}

			command := "glitter"
			usegit, args, err := parseReplay(replay)
			if err != nil {
				return err
			}
			if usegit {
				command = "git"
			}

			for _, arg := range args {
				colorize.Blue.Print("Executing command: ")
				fmt.Printf("%s %s\n", command, arg)
				shell.Command(command, strings.Fields(arg)...).Run()
			}

			return nil
		},
	}
}
