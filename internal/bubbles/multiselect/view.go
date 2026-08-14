package multiselect

import (
	"strings"

	tea "charm.land/bubbletea/v2"
	"charm.land/lipgloss/v2"
)

var (
	red   = lipgloss.NewStyle().Foreground(lipgloss.Red)
	blue  = lipgloss.NewStyle().Bold(true).Foreground(lipgloss.Blue)
	green = lipgloss.NewStyle().Foreground(lipgloss.Green)
)

func (m model) View() tea.View {
	var builder strings.Builder

	start, end := m.paginator.GetSliceBounds(len(m.elements))
	for i, item := range m.elements[start:end] {
		var checkBox, display string
		displayText := item.Display()

		if item.Selected() {
			checkBox = "[x] "
			display = green.Render(displayText)
		} else {
			checkBox = "[ ] "
			display = red.Render(displayText)
		}

		if m.cursor == start+i {
			builder.WriteString(blue.Render(checkBox))
		} else {
			builder.WriteString(checkBox)
		}

		builder.WriteString(display)
		builder.WriteRune('\n')
	}

	if m.paginator.TotalPages > 1 {
		for range perPage - (end - start) {
			builder.WriteRune('\n')
		}
		builder.WriteString(m.paginator.View())
		builder.WriteRune('\n')
	}

	return tea.NewView(builder.String())
}
