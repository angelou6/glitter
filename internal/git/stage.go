package git

import (
	"strings"

	"glitter/internal/shell"
)

func ParseStatus() []File {
	status, _ := shell.Command("git", "status", "--porcelain", "-uall").Output(false)
	statusFiles := strings.Split(strings.TrimRight(status, "\n"), "\n")

	var files []File

	for _, f := range statusFiles {
		elements := strings.Split(f, " ")
		files = append(files, File{
			Path:      elements[len(elements)-1],
			FullStr:   f,
			IsTracked: !strings.HasPrefix(f, " ") && !strings.HasPrefix(f, "?"),
		})
	}

	return files
}

func StagedFiles() []string {
	staged := []string{}
	files := ParseStatus()
	for _, f := range files {
		if f.IsTracked {
			staged = append(staged, f.Path)
		}
	}

	return staged
}

func Stage(files ...string) {
	args := []string{"add"}
	args = append(args, files...)
	shell.Command("git", args...).Run()
}

func Unstage(files ...string) error {
	args := []string{"restore", "--staged"}
	if !RepoHasCommits() {
		args = []string{"rm", "-r", "--cached"}
	}

	if len(files) == 0 {
		files = []string{"."}
	}
	args = append(args, files...)

	return shell.Command("git", args...).Run()
}
