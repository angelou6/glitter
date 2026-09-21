package commands

import (
	"glitter/internal/commands/clone"

	"github.com/urfave/cli/v3"
)

func NewRootCommand() *cli.Command {
	return &cli.Command{
		Name:                  "glitter",
		Usage:                 "Opinionated git shortcuts",
		EnableShellCompletion: true,
		Commands: []*cli.Command{
			initCommand(),
			publishCommand(),
			commitCommand(),
			pushCommand(),
			undoCommand(),
			stageCommand(),
			pullCommand(),
			openCommand(),
			clone.CloneCommand(),
		},
	}
}
