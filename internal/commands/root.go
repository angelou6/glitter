package commands

import "github.com/urfave/cli/v3"

func NewRootCommand() *cli.Command {
	return &cli.Command{
		Name:  "glitter",
		Usage: "Opinionated git shortcuts",
		Commands: []*cli.Command{
			newInitCommand(),
			newPublishCommand(),
			newCommitCommand(),
			newPushCommand(),
			newUndoCommand(),
			newAddCommand(),
			newPullCommand(),
			newOpenCommand(),
		},
	}
}
