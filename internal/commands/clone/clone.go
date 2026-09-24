package clone

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"sync"

	"glitter/internal/bubbles/multiselect"
	"glitter/internal/github"
	"glitter/internal/shell"

	"github.com/urfave/cli/v3"
)

func parseRepos() ([]Repo, error) {
	var userRepos []Repo
	repos, err := github.GetRepos()
	if err != nil {
		return []Repo{}, err
	}

	if err := json.Unmarshal([]byte(repos), &userRepos); err != nil {
		return []Repo{}, err
	}

	return userRepos, nil
}

func cloneAll(urls []string) {
	var wg sync.WaitGroup

	for _, url := range urls {
		wg.Go(func() {
			fmt.Printf("Cloning %s\n", url)
			if err := shell.Command("git", "clone", url).Silent().Run(); err != nil {
				fmt.Fprintf(os.Stderr, "Error cloning %s\n%v\n", url, err)
				return
			}
			fmt.Printf("Done cloning %s\n", url)
		})
	}
	wg.Wait()
}

func CloneCommand() *cli.Command {
	return &cli.Command{
		Name:      "clone",
		Usage:     "Clone a repository",
		ArgsUsage: "[repos]",
		Arguments: []cli.Argument{
			&cli.StringArgs{
				Name:      "repos",
				UsageText: "repos to clone, leave empty to clone from your github repos",
				Min:       0,
				Max:       -1,
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			repos := c.StringArgs("repos")
			if len(repos) > 0 {
				cloneAll(repos)
				return nil
			}

			userRepos, err := parseRepos()
			if err != nil {
				return err
			}
			multiselect.New(userRepos).Run()
			cloneAll(filter(userRepos))

			return nil
		},
	}
}
