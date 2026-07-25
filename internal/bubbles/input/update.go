package input

import tea "charm.land/bubbletea/v2"

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	var cmd tea.Cmd

	switch msg := msg.(type) {
	case tea.KeyPressMsg:
		switch msg.String() {
		case "ctrl+c", "ctrl+d":
			m.exitWithError = true
			return m, tea.Quit
		case "enter":
			if err := m.validate(m.textInput.Value()); err == nil {
				if m.textInput.Value() == "" {
					m.textInput.SetValue(m.textInput.Placeholder)
				}
				m.textInput.SetVirtualCursor(false)
				return m, tea.Quit
			}
		}
	}
	m.textInput, cmd = m.textInput.Update(msg)
	return m, cmd
}
