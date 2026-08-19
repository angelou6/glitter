package git

import (
	"glitter/internal/shell"
	"os"
)

func RepoHasCommits() bool {
	if _, err := shell.Command("git", "rev-parse", "--verify", "HEAD").Output(); err != nil {
		return false
	}
	return true
}

func IsRepo() bool {
	_, err := os.Stat(".git")
	return !os.IsNotExist(err)
}

func HasChanges() bool {
	out, _ := shell.Command("git", "diff").Output()
	return len(out) > 0
}

func MessagesToArgs(messages []string) []string {
	res := []string{}
	for _, m := range messages {
		res = append(res, "-m")
		res = append(res, m)
	}

	return res
}
