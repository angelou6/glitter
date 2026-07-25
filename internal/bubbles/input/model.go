package input

import (
	"errors"

	"charm.land/bubbles/v2/textinput"
	tea "charm.land/bubbletea/v2"
)

type model struct {
	prompt        string
	textInput     textinput.Model
	exitWithError bool
	validate      func(string) error
}

func (m model) Init() tea.Cmd {
	return nil
}

func New(prompt, defaultVal string, validation func(string) error) model {
	ti := textinput.New()
	ti.Prompt = prompt + ": "
	ti.Placeholder = defaultVal
	ti.SetWidth(100)
	ti.Focus()
	return model{
		textInput: ti,
		validate:  validation,
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
	return newModel.textInput.Value(), nil
}
