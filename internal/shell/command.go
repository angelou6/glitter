package shell

import (
	"os"
	"os/exec"
)

type shellCommand struct {
	cmd    *exec.Cmd
	silent bool
}

// Make command silent
func (s shellCommand) Silent() shellCommand {
	s.silent = true
	return s
}

func Command(command string, args ...string) shellCommand {
	cmd := exec.Command(command, args...)
	cmd.Stderr = os.Stderr

	return shellCommand{cmd, false}
}

// Run command
func (s shellCommand) Run() error {
	if s.silent {
		s.cmd.Stderr = nil
	} else {
		s.cmd.Stdout = os.Stdout
	}
	return s.cmd.Run()
}

// Run command and get output
// Using silence suppresses stderr
func (s shellCommand) Output() (string, error) {
	if s.silent {
		s.cmd.Stderr = nil
	}

	out, err := s.cmd.Output()
	if err != nil {
		return "", err
	}
	return string(out[:]), nil
}

// Run command detached from main process
func (s shellCommand) Spawn() error {
	if s.silent {
		s.cmd.Stderr = nil
	}
	return s.cmd.Start()
}

// Check if command exists
func Exists(command string) bool {
	if _, err := exec.LookPath(command); err != nil {
		return false
	}
	return true
}
