package replay

import (
	"glitter/internal/bubbles/confirm"
	"glitter/internal/bubbles/input"
	"glitter/internal/git"

	"github.com/d5/tengo/v2"
)

var Model = map[string]tengo.Object{
	"info":         &tengo.UserFunction{Name: "info", Value: info},
	"gen_messages": &tengo.UserFunction{Name: "gen_messages", Value: auto_messsage},
	"commit":       &tengo.UserFunction{Name: "commit", Value: commit},
	"push":         &tengo.UserFunction{Name: "push", Value: push},
	"init":         &tengo.UserFunction{Name: "init", Value: glitter_init},
	"confirm":      &tengo.UserFunction{Name: "confirm", Value: confirm_prompt},
	"input":        &tengo.UserFunction{Name: "input", Value: input_prompt},
	"stage":        &tengo.UserFunction{Name: "stage", Value: stage},
	"staged":       &tengo.UserFunction{Name: "staged", Value: staged},
}

func toStringSlice(arg tengo.Object) ([]string, error) {
	arr, ok := arg.(*tengo.Array)
	if !ok {
		return nil, tengo.ErrInvalidArgumentType{
			Name: "messages", Expected: "array", Found: arg.TypeName(),
		}
	}
	stringSlice := make([]string, len(arr.Value))
	for i, v := range arr.Value {
		stringSlice[i], ok = tengo.ToString(v)
		if !ok {
			return nil, tengo.ErrInvalidArgumentType{
				Name: "messages", Expected: "string", Found: v.TypeName(),
			}
		}
	}

	return stringSlice, nil
}

func info(args ...tengo.Object) (tengo.Object, error) {
	obj, err := tengo.FromInterface(map[string]any{
		"is_repo":     git.IsRepo(),
		"has_commits": git.RepoHasCommits(),
		"has_changes": git.HasChanges(),
		"origin":      git.Origin(),
	})
	return obj, err
}

func staged(args ...tengo.Object) (tengo.Object, error) {
	if len(args) > 0 {
		return nil, tengo.ErrWrongNumArguments
	}

	files := git.StagedFiles()
	arr := make([]tengo.Object, len(files))
	for i, b := range files {
		arr[i] = &tengo.String{Value: b}
	}
	return &tengo.Array{Value: arr}, nil
}

func auto_messsage(args ...tengo.Object) (tengo.Object, error) {
	if len(args) > 0 {
		return nil, tengo.ErrWrongNumArguments
	}

	messages := git.AutoGenerateMessage()
	arr := make([]tengo.Object, len(messages))
	for i, b := range messages {
		arr[i] = &tengo.String{Value: b}
	}
	return &tengo.Array{Value: arr}, nil
}

func push(args ...tengo.Object) (tengo.Object, error) {
	if len(args) != 3 {
		return nil, tengo.ErrWrongNumArguments
	}

	messages, err := toStringSlice(args[0])
	if err != nil {
		return nil, err
	}

	// Error is ignored because tengo.ToBool converts using thruthy values
	// Fails anyways, I think
	force, _ := tengo.ToBool(args[1])
	all, _ := tengo.ToBool(args[2])

	return nil, git.Push(messages, force, all)
}

func commit(args ...tengo.Object) (tengo.Object, error) {
	if len(args) != 2 {
		return nil, tengo.ErrWrongNumArguments
	}

	messages, err := toStringSlice(args[0])
	if err != nil {
		return nil, err
	}
	all, _ := tengo.ToBool(args[1])

	return nil, git.StageAndCommit(messages, all)
}

func stage(args ...tengo.Object) (tengo.Object, error) {
	if len(args) != 2 {
		return nil, tengo.ErrWrongNumArguments
	}

	files, err := toStringSlice(args[0])
	if err != nil {
		return nil, err
	}

	unstage, _ := tengo.ToBool(args[1])
	if unstage {
		return nil, git.Unstage(files...)
	}
	return nil, git.Stage(files...)
}

func glitter_init(args ...tengo.Object) (tengo.Object, error) {
	if len(args) != 1 {
		return nil, tengo.ErrWrongNumArguments
	}

	branch, ok := tengo.ToString(args[1])
	if !ok {
		return nil, tengo.ErrInvalidArgumentType{
			Name: "branch", Expected: "string", Found: args[1].TypeName(),
		}
	}

	return nil, git.Initialize(branch)
}

func input_prompt(args ...tengo.Object) (tengo.Object, error) {
	if len(args) != 1 {
		return nil, tengo.ErrWrongNumArguments
	}

	prompt, ok := tengo.ToString(args[0])
	if !ok {
		return nil, tengo.ErrInvalidArgumentType{
			Name: "prompt", Expected: "string", Found: args[1].TypeName(),
		}
	}

	res, err := input.New(prompt, "", func(s string) error { return nil }).Run()
	if err != nil {
		return nil, err
	}

	return &tengo.String{Value: res}, nil
}

func confirm_prompt(args ...tengo.Object) (tengo.Object, error) {
	if len(args) != 1 {
		return nil, tengo.ErrWrongNumArguments
	}

	prompt, ok := tengo.ToString(args[0])
	if !ok {
		return nil, tengo.ErrInvalidArgumentType{
			Name: "prompt", Expected: "string", Found: args[1].TypeName(),
		}
	}

	res, err := confirm.Run(prompt)
	if err != nil {
		return nil, err
	}

	if res {
		return tengo.TrueValue, nil
	}
	return tengo.FalseValue, nil
}
