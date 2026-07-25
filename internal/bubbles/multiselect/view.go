package multiselect

import (
	"glitter/internal/colorize"
	"strings"

	tea "charm.land/bubbletea/v2"
)

func (m model) View() tea.View {
	var builder strings.Builder

	start, end := m.paginator.GetSliceBounds(len(m.elements))
	for i, item := range m.elements[start:end] {
		var checkBox string
		var display string
		displayText := item.Display()

		if item.Selected() {
			checkBox = "[x] "
			display = colorize.Green.Tint(displayText)
		} else {
			checkBox = "[ ] "
			display = colorize.Red.Tint(displayText)
		}

		if m.cursor == start+i {
			builder.WriteString(colorize.Magenta.Tint(checkBox))
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
