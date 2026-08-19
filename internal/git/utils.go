package git

import (
	"fmt"
	"slices"
	"strings"

	"glitter/internal/shell"
)

func Origin() string {
	remote, err := shell.Command("git", "remote", "get-url", "origin").Output(true)
	if err != nil {
		return ""
	}
	return strings.TrimRight(remote, "\n")
}

func RepoHasCommits() bool {
	if _, err := shell.Command("git", "rev-parse", "--verify", "HEAD").Output(true); err != nil {
		return false
	}
	return true
}

func IsRepo() bool {
	return shell.DirExists(".git")
}

func HasChanges() bool {
	out, err := shell.Command("git", "status", "--porcelain").Output(true)
	if err != nil {
		return false
	}
	return len(out) > 0
}

func MessagesToArgs(messages []string) []string {
	res := []string{}
	for _, m := range messages {
		res = append(res, "-m "+m)
	}

	return res
}

func HasUnpushedCommits() bool {
	log, _ := shell.Command("git", "log", "@{u}..", "--oneline").Output(false)
	return len(log) > 0
}

func AutoGenerateMessage() []string {
	args := []string{"commit"}
	staged := StagedFiles()

	stagedLen := len(staged)
	if stagedLen == 1 {
		return append(args, fmt.Sprintf("-m Changed %s", staged[0]))
	}

	return slices.Concat(args, []string{
		fmt.Sprintf("-m Changed %d files", stagedLen),
		fmt.Sprintf("-m Files changed: %s", strings.Join(staged, ", ")),
	})
}
