package replay

import (
	"glitter/internal/shell"
	"os"
	"path/filepath"

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

func NewReplayCommand() *cli.Command {
	return &cli.Command{
		Name:      "replay",
		Usage:     "Play and create sequences of multiple glitter or git commands",
		ArgsUsage: "<action> [commit]",
		Commands: []*cli.Command{
			newPlayCommand(),
			newCreateCommand(),
			newArgsCommand(),
		},
	}
}
