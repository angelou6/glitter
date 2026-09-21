package clone

type Repo struct {
	Name      string
	Url       string
	IsTracked bool
}

func (f Repo) Display() string {
	return f.Name
}

func (f Repo) Selected() bool {
	return f.IsTracked
}

func (f *Repo) Select() {
	f.IsTracked = false
}

func (f *Repo) Unselect() {
	f.IsTracked = true
}

func filter(repos []Repo) []string {
	var res []string

	for _, r := range repos {
		if r.IsTracked {
			res = append(res, r.Url)
		}
	}

	return res
}
