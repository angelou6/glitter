package git

import (
	"glitter/internal/shell"
)

func RepoHasCommits() bool {
	if _, err := shell.Command("git", "rev-parse", "--verify", "HEAD").Output(); err != nil {
		return false
	}
	return true
}

func IsRepo() bool {
	return shell.DirExists(".git")
}

func HasChanges() bool {
	out, _ := shell.Command("git", "status", "--porcelain").Output()
	return len(out) > 0
}

func MessagesToArgs(messages []string) []string {
	res := []string{}
	for _, m := range messages {
		res = append(res, "-m "+m)
	}

	return res
}

func hasUnpushedCommits() bool {
	log, _ := shell.Command("git", "log", "@{u}..", "--oneline").Output()
	return len(log) > 0
}
