package options

import tea "charm.land/bubbletea/v2"

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyPressMsg:
		switch msg.String() {
		case "ctrl+c", "ctrl+d", "q", "esc":
			m.exitWithError = true
			return m, tea.Quit
		case "enter":
			return m, tea.Quit
		case "down", "j":
			m.cursor = min(len(m.options)-1, m.cursor+1)
		case "up", "k":
			m.cursor = max(0, m.cursor-1)
		}
	}
	return m, nil
}
