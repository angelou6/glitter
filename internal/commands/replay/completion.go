package replay

import (
	"context"
	"fmt"
	"os"

	"github.com/urfave/cli/v3"
)

func getReplayEntries() ([]os.DirEntry, error) {
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

func fileCompletion(_ context.Context, c *cli.Command) {
	if c.NArg() > 0 {
		return
	}

	entries, _ := getReplayEntries()
	for _, e := range entries {
		fmt.Println(e.Name())
	}
}
