package options

import (
	"fmt"
	"glitter/internal/colorize"
	"strings"

	tea "charm.land/bubbletea/v2"
)

func (m model) View() tea.View {
	var builder strings.Builder

	fmt.Fprintf(&builder, "%s:\n", m.prompt)
	for i, item := range m.options {
		if i == m.cursor {
			builder.WriteString(colorize.Magenta.Tint("> "))
		} else {
			builder.WriteString("  ")
		}
		builder.WriteString(item)
		builder.WriteRune('\n')
	}

	return tea.NewView(builder.String())
}
