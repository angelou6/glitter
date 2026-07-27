package confirm

import "glitter/internal/bubbles/options"

func Run(prompt string) (bool, error) {
	res, err := options.New(prompt, []string{"yes", "no"}).Run()
	if err != nil {
		return false, err
	}
	return res == "yes", nil
}
