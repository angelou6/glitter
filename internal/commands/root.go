package commands

import (
	"glitter/internal/commands/replay"

	"github.com/urfave/cli/v3"
)

func NewRootCommand() *cli.Command {
	return &cli.Command{
		Name:                  "glitter",
		Usage:                 "Opinionated git shortcuts",
		EnableShellCompletion: true,
		Commands: []*cli.Command{
			newInitCommand(),
			newPublishCommand(),
			newCommitCommand(),
			newPushCommand(),
			newUndoCommand(),
			newStageCommand(),
			newPullCommand(),
			newOpenCommand(),
			replay.NewReplayCommand(),
		},
	}
}
