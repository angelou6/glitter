package git

import (
	"fmt"
	"glitter/internal/shell"
	"strings"
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

	stagedFiles := len(staged)
	plural := ""
	if stagedFiles != 1 {
		plural = "s"
	}

	return append(args, MessagesToArgs([]string{
		fmt.Sprintf("Changed %d file%s", stagedFiles, plural),
		fmt.Sprintf("Files changed: %s", strings.Join(staged, ", ")),
	})...)
}
