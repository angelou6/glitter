# glitter ✨

Opinionated Git shortcuts. Designed to streamline, not replace.

## Installation

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

## Commands

### init

Initialize git repo

```
Usage: glitter init [OPTIONS] [COMMAND]

Commands:
  publish  Publish online, public by default
  help     Print this message or the help of the given subcommand(s)

Options:
  -m, --message <MESSAGE>  Commit message
  -f, --force              Force commit to execute
  -h, --help               Print help
```

`Publish` the local repo, with commands or a simple TUI. Public by default.

```
Usage: glitter init publish [OPTIONS]

Options:
  -n, --name <NAME>  Name of repo
  -d, --desc <DESC>  Description of repo
  -p, --private      Repo visibility
  -h, --help         Print help
```

### push

Stage, push and commit all in one.

```
Usage: glitter push [OPTIONS] [COMMAND]

Commands:
  undo  Undo latest
  help  Print this message or the help of the given subcommand(s)

Options:
  -m, --message <MESSAGE>  Commit message
      --amend              Amend all new modifications to latest
  -f, --force              Force command to execute
  -a, --all                Ignore staged files and stage all
  -h, --help               Print help
```

`Undo` the last push:

```
Usage: glitter push undo [OPTIONS]

Options:
      --hard  Undo hard
  -h, --help  Print help
```

### commit

Stage all files and commit.

Accepts the same flags as `push`.

### add

Opens an interactive TUI to stage files. You can also use it as the normal `git add`

Navigate with `j`/`k` or arrow keys, toggle staging with `Space` or `Enter`, and quit with `q` or `Esc`.

```
Usage: glitter add [OPTIONS] [FILES]...

Arguments:
  [FILES]...

Options:
  -r, --revert  Revert
  -h, --help    Print help
```

### pull

Force pull and reset local changes

```
Usage: glitter pull [OPTIONS]

Options:
  -y, --yes   Skip warning
  -h, --help  Print help
```

### open

Open the repository in the default browser.

```
Usage: glitter open [OPTIONS] [COMMIT]

Arguments:
  [COMMIT]  Open a specific commit

Options:
  -d, --dump  Print the URL instead of opening it
  -h, --help  Print help
```
