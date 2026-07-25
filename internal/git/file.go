package git

type File struct {
	Path, FullStr string
	IsTracked     bool
}

func (f File) Display() string {
	return f.Path
}

func (f File) Selected() bool {
	return f.IsTracked
}

func (f *File) Stage() {
	Unstage(f.Path)
	f.IsTracked = false
}

func (f *File) Unstage() {
	Stage(f.Path)
	f.IsTracked = true
}
