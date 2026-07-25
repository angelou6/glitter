package options

import (
	"errors"

	tea "charm.land/bubbletea/v2"
)

type model struct {
	options       []string
	prompt        string
	exitWithError bool
	cursor        int
}

func (m model) Init() tea.Cmd {
	return nil
}

func New(prompt string, options []string) model {
	return model{
		prompt:  prompt,
		options: options,
	}
}

func (m model) Run() (string, error) {
	p := tea.NewProgram(m)
	final, err := p.Run()
	if err != nil {
		return "", err
	}
	newModel := final.(model)
	if newModel.exitWithError {
		return "", errors.New("Keyboard interrupt")
	}
	return newModel.options[newModel.cursor], nil
}
