# glitter ✨

Opinionated Git shortcuts. Designed to streamline, not replace.

## Installation

On `/usr/local/bin/glitter`
```bash
sudo make install
```

On `~/.local/bin`:
```bash
make local_install
```

## Commands

### `push`

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

**Undo** the last push:
```
Usage: glitter push undo [OPTIONS]

Options:
      --hard  Undo hard
  -h, --help  Print help
```


### `commit`

Stage all files and commit.

Accepts the same flags as `push`.


### `add`

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


### `pull`

Force pull and reset local changes

```
Usage: glitter pull [OPTIONS]

Options:
  -y, --yes   Skip warning
  -h, --help  Print help
```


### `open`

Open the repository in the default browser.

```
Usage: glitter open [OPTIONS] [COMMIT]

Arguments:
  [COMMIT]  Open a specific commit

Options:
  -d, --dump  Print the URL instead of opening it
  -h, --help  Print help
```
