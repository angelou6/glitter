package replay

import (
	"os"
	"path/filepath"

	"glitter/internal/shell"

	"github.com/urfave/cli/v3"
)

func getConfigDir() (string, error) {
	home, _ := os.UserConfigDir()
	confDir := filepath.Join(home, "glitter")
	if !shell.DirExists(confDir) {
		if err := os.Mkdir(confDir, 0o755); err != nil {
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
		Usage:     "Play replays",
		ArgsUsage: "<action>",
		Commands: []*cli.Command{
			newPlayCommand(),
			newOpenCommand(),
		},
	}
}
