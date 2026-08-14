package options

import (
	"fmt"
	"strings"

	tea "charm.land/bubbletea/v2"
	"charm.land/lipgloss/v2"
)

var selected = lipgloss.NewStyle().Bold(true).Foreground(lipgloss.Magenta)

func (m model) View() tea.View {
	var builder strings.Builder

	fmt.Fprintf(&builder, "%s:\n", m.prompt)
	for i, item := range m.options {
		if i == m.cursor {
			builder.WriteString(selected.Render("> "))
		} else {
			builder.WriteString("  ")
		}
		builder.WriteString(item)
		builder.WriteRune('\n')
	}

	return tea.NewView(builder.String())
}
