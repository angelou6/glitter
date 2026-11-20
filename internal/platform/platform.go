package platform

import (
	"glitter/internal/shell"
)

type Platform int

const (
	GitHub Platform = iota
	Codeberg
)

func GithubCliExists() bool {
	return shell.Exists("gh")
}

func ForgejoCliExists() bool {
	return shell.Exists("fj")
}

// Get repositories from platform. GitHub only
func GetRepos() (string, error) {
	return shell.Command("gh", "repo", "ls", "--json", "name,url").Output()
}

func Publish(platform Platform, name, desc string, private bool) error {
	args := []string{"repo", "create", name, "--description", desc, "--remote=origin", "--push"}

	if private {
		args = append(args, "--private")
	}

	if platform == Codeberg {
		args = append(args, "--host", "https://codeberg.org/")
		return shell.Command("fj", args...).Run()
	} else {
		if !private {
			args = append(args, "--public")
		}
		args = append(args, "--source", ".")
		return shell.Command("gh", args...).Run()
	}
}
