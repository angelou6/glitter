# gitfuckyou

A minimalist, aggressive Git helper.

> [!WARNING]
> This tool uses `--force` and `reset --hard`. It **will** delete data if you are not careful. Use it at your own risk.

## Installation

```bash
sudo make install
```

## Usage

```bash
gitfuckyou <command> [arguments]
```

### Commands

#### `push`
Force push changes with an optional blame. It automatically adds all files (`git add .`) and commits them before pushing.

- `-m "message"`: Custom commit message (default: "fuck you")
- `-blame "Name <email>"`: Set a specific author for the commit.
- `-last`: Amend all new modifications to the latest push instead of creating a new one.

**Example:**
```bash
gitfuckyou push -m "Fixed the bug" -blame "Coworker <coworker@company.com>"
```

#### `pull`
Force pull and reset local changes. This will wipe all unsaved changes and sync with the remote.

- `-fu`: Skip the "Are you sure?" warning (Fuck You mode).

**Example:**
```bash
gitfuckyou pull -fu
```

## Help

Use the `-h` flag with any command to see more information:

```bash
gitfuckyou push -h
gitfuckyou pull -h
```
