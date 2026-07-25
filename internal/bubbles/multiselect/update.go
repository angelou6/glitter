package multiselect

import tea "charm.land/bubbletea/v2"

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tea.KeyPressMsg:
		switch msg.String() {
		case "ctrl+c", "ctrl+d", "q", "esc":
			return m, tea.Quit
		case "a":
			for _, element := range m.elements {
				if element.Selected() {
					element.Stage()
				} else {
					element.Unstage()
				}
			}
		case "enter", "space":
			if m.elements[m.cursor].Selected() {
				m.elements[m.cursor].Stage()
			} else {
				m.elements[m.cursor].Unstage()
			}
		case "down", "j":
			m.cursor = min(len(m.elements)-1, m.cursor+1)
			m.paginator.Page = m.cursor / m.paginator.PerPage
		case "up", "k":
			m.cursor = max(0, m.cursor-1)
			m.paginator.Page = m.cursor / m.paginator.PerPage
		}
	}

	var cmd tea.Cmd
	m.paginator, cmd = m.paginator.Update(msg)
	start, end := m.paginator.GetSliceBounds(len(m.elements))
	m.cursor = max(0, min(max(m.cursor, start), end-1))

	return m, cmd
}
