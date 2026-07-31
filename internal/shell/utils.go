package shell

import (
	"errors"
	"os"
	"runtime"
)

func DirExists(dir string) bool {
	_, err := os.Stat(dir)
	return !os.IsNotExist(err)
}

func Open(url string) error {
	switch runtime.GOOS {
	case "linux":
		return Command("xdg-open", url).Spawn()
	case "windows":
		return Command("cmd", "/c", "start", url).Spawn()
	case "darwin":
		return Command("open", url).Spawn()
	}
	return errors.New("OS not found")
}
