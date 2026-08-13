# Replays

Replays are located in `~/.config/glitter` and use the [tengo scripting language](https://github.com/d5/tengo).

Tengo is extended with extra Glitter functionality.

```go
fmt := import("fmt")
glitter := import("glitter")

fmt.println(glitter.info()) // Info about the repo
fmt.println(glitter.gen_messages()) // Auto generates commit message from staged files
fmt.println(glitter.staged()) // Array with staged files

// arg1: message, arg2: Ignore staged files and stage all
glitter.commit(["cool", "commit"], true)

// arg1: message, arg2: Force command to execute, arg3: Ignore staged files and stage all
glitter.push(["cool", "commit"], false, true)

// arg1: message, arg2: branch
glitter.init(["cool", "commit"], "main")
```

You are also given the arguments passed to the CLI when making the call.

```sh
glitter replay play myreplay arg1 arg2
```

`myreplay`

```go
fmt := import("fmt")

fmt.println(args) // Output: ["arg1", "arg2"]
```

You can run shell commands using `exec`.

```go
fmt := import("fmt")

out := exec("echo", "hello")
fmt.print(out) // Output: hello
```

Prompts for confirmation and user input are also included.

```go
fmt := import("fmt")
glitter := import("glitter")

fmt.println(glitter.input("my question")) // Returns the input from the user as a string
fmt.println(glitter.confirm("my question")) // Boolean
```
