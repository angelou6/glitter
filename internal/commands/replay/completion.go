package replay

import (
	"context"
	"fmt"

	"github.com/urfave/cli/v3"
)

func fileCompletion(ctx context.Context, c *cli.Command) {
	if c.NArg() > 0 {
		return
	}

	entries, _ := getConfigEntries()
	for _, e := range entries {
		fmt.Println(e.Name())
	}
}
