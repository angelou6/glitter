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

#### `push`
Force push changes. It automatically adds all files (`git add .`) and commits them before pushing.

- `-m "message"`: Custom commit message
- `--last`: Amend all new modifications to the latest push instead of creating a new one.
- `--force`: Forces the push to happen even without a commit message.

**Example:**
```bash
glitter push -m "Fixed the bug"
```

#### `pull`
Force pull and reset local changes. This will wipe uncommited changes and sync with the remote.

- `-y`: Skip the "Are you sure?" warning.

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
