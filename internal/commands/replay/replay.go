package replay

import (
	"context"
	"fmt"
	"glitter/internal/colorize"
	"glitter/internal/shell"
	"os"
	"path/filepath"
	"strings"

	"github.com/urfave/cli/v3"
)

func getConfigDir() (string, error) {
	home, _ := os.UserConfigDir()
	confDir := filepath.Join(home, "glitter")
	if !shell.DirExists(confDir) {
		if err := os.Mkdir(confDir, 0755); err != nil {
			return "", err
		}
	}

	return confDir, nil
}

func getFullDir(replay string) (string, error) {
	confDir, err := getConfigDir()
	if err != nil {
		return "", err
	}
	return filepath.Join(confDir, replay), nil
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

func NewReplayCommand() *cli.Command {
	return &cli.Command{
		Name:      "replay",
		Usage:     "Play and create sequences of multiple glitter or git commands",
		ArgsUsage: "<action> [commit]",
		Commands: []*cli.Command{
			newCreateCommand(),
			newArgsCommand(),
		},
		Arguments: []cli.Argument{
			&cli.StringArg{
				Name:      "replay",
				UsageText: "Name of the replay to play",
			},
		},
		Flags: []cli.Flag{
			&cli.BoolFlag{
				Name:    "remove",
				Aliases: []string{"r"},
				Usage:   "remove a replay",
			},
			&cli.BoolFlag{
				Name:    "open",
				Aliases: []string{"o"},
				Usage:   "open a replay in the default text editor",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			command := "glitter"
			replay := c.StringArg("replay")
			if replay == "" {
				confDir, err := getConfigDir()
				if err != nil {
					return err
				}
				entries, err := os.ReadDir(confDir)
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

			remove := c.Bool("remove")
			open := c.Bool("open")

			dir, err := getFullDir(replay)
			if err != nil {
				return err
			}

			switch {
			case open:
				return shell.Open(dir)
			case remove:
				if err := os.Remove(dir); err != nil {
					return err
				}
				fmt.Println("Deleted successfully")
			default:
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
			}

			return nil
		},
	}
}
