# ✨ Glitter

<p align="center">
  <img src="https://github.com/angelou6/glitter/blob/master/assets/glitter_logo.png?raw=true" alt="Sublime's custom image"/>
</p>

---

`Glitter` is a tool providing some shorcuts that I found useful working in solo projects.

## Why?
Git can be very verbose some times and executing multiple commands just to do one thing gets tiring.

**For example**:

From this:
```sh
git add .
git commit -m "my cool project"
git push
```

To this.
```sh
glitter push -m "my cool project"
```

Or this:
```sh
git init
git add .
git commit -m "initial commit"
git branch -M main
git remote add origin https://github.com/angelou6/glitter.git
git push -u origin main
```

To this:
```sh
glitter init -r https://github.com/angelou6/glitter.git publish
```

---

`Glitter` also has some small TUIs to help with publishing and staging:

| Publish                                | Stage                                     |
| ---                                    | ---                                       |
|![stage tui](/assets/glitter_stage.png) | ![stage tui](/assets/glitter_publish.png) |

## Usage

This is an example of going from local files to pushing to github and making a commit with fixes using `Glitter`.

```sh
glitter init -m "my cool repo" publish -n glitter
glitter push -m "fix: it compiles now"
```

## Instalation

### Linux

On `/usr/local/bin/glitter`

```bash
sudo make install
```

On `~/.local/bin`:

```bash
make local_install
```

### Windows / Mac

I don't use either of them, but I assume this will work.

If there are any errors in either of them, PRs are welcome.

```
cargo install --path .
```

## Dependencies

* git
* github-cli (optional for publishing on github)
