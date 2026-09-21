package github

import "glitter/internal/shell"

func GetRepos() (string, error) {
	return shell.Command("gh", "repo", "ls", "--json", "name,url").Output()
}

func Publish(name, desc string, private bool) error {
	args := []string{"repo", "create", name, "--description", desc, "--source", ".", "--remote=origin", "--push"}
	if private {
		args = append(args, "--private")
	} else {
		args = append(args, "--public")
	}

	return shell.Command("gh", args...).Run()
}
