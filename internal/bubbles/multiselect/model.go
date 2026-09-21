package multiselect

import (
	"charm.land/bubbles/v2/paginator"
	tea "charm.land/bubbletea/v2"
)

const perPage = 10

type Element interface {
	Display() string
	Selected() bool
	Select()
	Unselect()
}

type model struct {
	elements  []Element
	cursor    int
	paginator paginator.Model
}

func (m model) Init() tea.Cmd {
	return nil
}

func New[T any, PT interface {
	*T
	Element
}](elements []T) model {
	p := paginator.New(paginator.WithPerPage(perPage))
	p.Type = paginator.Dots
	p.SetTotalPages(len(elements))

	elems := make([]Element, len(elements))
	for i := range elements {
		elems[i] = PT(&elements[i])
	}

	return model{elements: elems, paginator: p}
}

func (m model) Run() error {
	p := tea.NewProgram(m)
	if _, err := p.Run(); err != nil {
		return err
	}
	return nil
}
