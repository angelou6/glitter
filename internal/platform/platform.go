package platform

import (
	"errors"
	"glitter/internal/shell"
)

type Platform int

const (
	GitHub Platform = iota
	Codeberg
)

func CheckExistance(use_codeberg bool) error {
	if use_codeberg && !shell.Exists("fj") {
		return errors.New("forgejo-cli is missing")
	} else if !shell.Exists("gh") {
		return errors.New("github-cli is missing")
	}
	return nil
}

// Get repositories from platform
// GitHub only
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
