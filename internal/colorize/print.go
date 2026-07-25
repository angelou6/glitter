package colorize

import (
	"fmt"
	"strings"
)

func joinAny(arr []any) string {
	var b strings.Builder
	for _, a := range arr {
		fmt.Fprint(&b, a)
	}

	return b.String()
}

func (c Color) Println(a ...any) {
	fmt.Printf("\x1b[%dm%s\x1b[0m\n", c, joinAny(a))
}

func (c Color) Print(a ...any) {
	fmt.Printf("\x1b[%dm%s\x1b[0m", c, joinAny(a))
}

func (c Color) Fprint(a ...any) string {
	return fmt.Sprintf("\x1b[%dm%s\x1b[0m", c, joinAny(a))
}

func (c Color) Tint(s string) string {
	return fmt.Sprintf("\x1b[%dm%s\x1b[0m", c, s)
}
