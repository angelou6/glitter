package shell

import (
	"os"
	"os/exec"
)

type shellCommand struct {
	cmd *exec.Cmd
}

func Command(args ...string) shellCommand {
	cmd := exec.Command(args[0], args[1:]...)
	cmd.Stderr = os.Stderr

	return shellCommand{cmd}
}

// Run command
func (s shellCommand) Run() error {
	s.cmd.Stdout = os.Stdout
	return s.cmd.Run()
}

// Run command and get output
func (s shellCommand) Output() (string, error) {
	out, err := s.cmd.Output()
	if err != nil {
		return "", err
	}
	return string(out[:]), nil
}

// Run command detached from main process
func (s shellCommand) Spawn() error {
	return s.cmd.Start()
}

// Check if command exists
func Exists(command string) bool {
	if _, err := exec.LookPath(command); err != nil {
		return false
	}
	return true
}
