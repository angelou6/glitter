# glitter

> [!WARNING]
> This tool uses `--force` and `reset --hard`. It **will** delete data if you are not careful. Use it at your own risk.

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
Force push changes with an optional blame. It automatically adds all files (`git add .`) and commits them before pushing.

- `-m "message"`: Custom commit message
- `-blame "Name <email>"`: Set a specific author for the commit.
- `-last`: Amend all new modifications to the latest push instead of creating a new one.
- `-force`: Forces the push to happen even without a commit message.

**Example:**
```bash
glitter push -m "Fixed the bug" -blame "Coworker <coworker@company.com>"
```

#### `pull`
Force pull and reset local changes. This will wipe all unsaved changes and sync with the remote.

- `-y`: Skip the "Are you sure?" warning.

**Example:**
```bash
glitter pull -y
```

## Help

Use the `-h` flag with any command to see more information:

```bash
glitter push -h
glitter pull -h
```
