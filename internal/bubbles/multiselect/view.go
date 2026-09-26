package multiselect

import (
	"strings"

	tea "charm.land/bubbletea/v2"
	"charm.land/lipgloss/v2"
)

var (
	orange = lipgloss.NewStyle().Foreground(lipgloss.Color("202")).Render
	blue   = lipgloss.NewStyle().Bold(true).Foreground(lipgloss.Blue).Render
	green  = lipgloss.NewStyle().Foreground(lipgloss.Green).Render
	grey   = lipgloss.NewStyle().Foreground(lipgloss.BrightBlack).Render
)

func (m model) View() tea.View {
	var builder strings.Builder

	start, end := m.paginator.GetSliceBounds(len(m.elements))
	builder.WriteString(grey("Return to accept | Space to toggle | 'a' to toggle all"))
	builder.WriteRune('\n')
	for i, item := range m.elements[start:end] {
		checkBox := "[ ] "
		display := orange(item.Display())

		if item.Selected() {
			checkBox = "[x] "
			display = green(item.Display())
		}

		if m.cursor == start+i {
			builder.WriteString(blue(checkBox))
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
