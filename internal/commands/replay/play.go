package replay

import (
	"context"
	"errors"
	"os"

	"glitter/internal/shell"

	"github.com/d5/tengo/v2"
	"github.com/d5/tengo/v2/stdlib"
	"github.com/urfave/cli/v3"
)

func execWrapper(args ...tengo.Object) (tengo.Object, error) {
	commandObj, ok := args[0].(*tengo.String)
	if !ok {
		return nil, errors.New("Invalid argument for command")
	}

	command := commandObj.Value

	var cmdArgs []string
	for _, arg := range args[1:] {
		argObj, ok := arg.(*tengo.String)
		if !ok {
			return nil, errors.New("Invalid argument for args")
		}
		cmdArgs = append(cmdArgs, argObj.Value)
	}

	out, err := shell.Command(command, cmdArgs...).Output(false)
	if err != nil {
		return nil, err
	}

	return &tengo.String{Value: string(out)}, nil
}

func newPlayCommand() *cli.Command {
	return &cli.Command{
		Name:          "play",
		Usage:         "Play a replay",
		ArgsUsage:     "<action>",
		ShellComplete: fileCompletion,
		Action: func(ctx context.Context, c *cli.Command) error {
			if c.Args().Len() == 0 {
				cli.ShowSubcommandHelpAndExit(c, 1)
			}

			dir, _ := getFullDir(c.Args().First())
			file, err := os.ReadFile(dir)
			if err != nil {
				return err
			}
			script := tengo.NewScript(file)

			args := c.Args().Slice()[1:]
			argsSlice := make([]any, len(args))
			for i, a := range args {
				argsSlice[i] = a
			}
			script.Add("args", argsSlice)
			script.Add("exec", execWrapper)
			moduleMap := stdlib.GetModuleMap("fmt")
			moduleMap.AddBuiltinModule("glitter", Model)
			script.SetImports(moduleMap)

			_, err = script.Run()

			return err
		},
	}
}
