package main

import (
	"context"
	"fmt"
	"glitter/internal/commands"
	"os"

	"github.com/urfave/cli/v3"
)

func init() {
	cli.CommandHelpTemplate += `{{if .Arguments}}
ARGUMENTS:
   {{range .Arguments}}{{.Name}}{{"\t"}}{{.UsageText}}{{if .Value}} [default: {{.Value}}]{{end}}
   {{end}}{{end}}`
}

func main() {
	if err := commands.NewRootCommand().Run(context.Background(), os.Args); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}
