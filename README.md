# ✨ Glitter

<p align="center">
  <img src="https://github.com/angelou6/glitter/blob/main/assets/glitter_logo.png?raw=true" alt="glitter logo"/>
</p>

---

`Glitter` is a tool that provides some shorcuts that I found useful for solo projects.

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
glitter init
glitter publish -o https://github.com/angelou6/glitter.git
```

---

`Glitter` also has some small TUIs to help with publishing and staging:

| Stage                                   | Publish                                   |
| --------------------------------------- | ----------------------------------------- |
| ![stage tui](/assets/glitter_stage.png) | ![stage tui](/assets/glitter_publish.png) |

## Example usage

```sh
# Initializing the repository and pushing it to GitHub
glitter init -m "my cool repo"
glitter publish -n glitter

# Pushing a fix
glitter push -m "fix: it compiles now"

# Amending the last push with new stuff
glitter push --amend

# Removing the lates push from remote
glitter undo push

# Opening the project in the default browser
glitter open
```

## Instalation

### Linux

On `/usr/local/bin/`

```bash
sudo make install
```

On `~/.local/bin`:

```bash
make local_install
```

### Windows / Mac

I don't use either of them, but I assume this will work.

If there are any errors, PRs are welcome.

```
cargo install --path .
```

## Dependencies

- git
- github-cli (optional for publishing on github)
