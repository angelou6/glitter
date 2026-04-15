# glitter

## Installation

```bash
sudo make install
```

## Usage

```bash
glitter <command> [arguments]
```

### Commands

#### `commit`
Stage all files and commit. 

- `-m "message"`, `--message "message"`: Commit message
- `--amend`: Amend all new modifications to the latest commit instead of creating a new one
- `-f`, `--force`: Force commit even without a message

**Examples:**
```bash
glitter commit -m "Fixed the bug"
glitter commit --amend
```

#### `push`
Stage, commit, and push changes.

- `-m "message"`, `--message "message"`: Commit message
- `--amend`: Amend all new modifications to the latest push instead of creating a new one
- `-f`, `--force`: Force push even without a commit message

**Examples:**
```bash
glitter push -m "Fixed the bug"
glitter push --amend
```

#### `pull`
Force pull and reset local changes. This will wipe uncommited changes and sync with the remote.

- `-y`, `--yes`: Skip the "Are you sure?" warning.

**Example:**
```bash
glitter pull -y
```

#### `open`
Open the project in the default web browser. If a commit is provided, opens that specific commit instead.

- `[commit]`: Open a specific commit
- `--dump`: Print the URL instead of opening it

**Examples:**
```bash
glitter open
glitter open HEAD
```

## Help

Use the `-h` flag with any command to see more information:

```bash
glitter push -h
glitter pull -h
```
