# Teleproj

**Teleproj** is a lightweight Rust-based project jumper that lets you quickly switch (`cd`) into your saved project directories from anywhere.
It works by maintaining a simple config file (`~/.teleproj.toml`) with your project paths, and integrates into your shell with a wrapper function for seamless navigation.

---

## Features

* 🚀 Jump into projects by **index** or **name**
* ➕ Add new project paths (`--add`)
* ➖ Remove paths by index (`--remove`)
* 📋 List all saved projects (`--list`)
* 🔍 Fuzzy matching for project names

---

## Installation

1. Clone and build:

   ```sh
   git clone https://github.com/spmaris/teleproj.git
   cd teleproj
   cargo build --release
   ```

2. Move the binary into your `$HOME/.local/bin` (or anywhere in `$PATH`):

   ```sh
   mkdir -p ~/.local/bin
   ln -s "$(pwd)/target/release/teleproj" ~/.local/bin/teleproj
   ```

3. Add `~/.local/bin` to your `PATH` if not already there:

   ```sh
   echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
   source ~/.zshrc
   ```

---

## Shell Integration

Add this helper function to your shell config (`~/.zshrc` or `~/.bashrc`):

```sh
tp() {
    output="$(teleproj "$@")"
    if [ -d "$output" ]; then
        cd "$output"
    else
        echo "$output"
    fi
}
```

Reload your shell:

```sh
source ~/.zshrc
```

Now you can use `tp` to jump into projects.

---

## Usage

```sh
Usage: teleproj [OPTIONS] [PROJECT]

Arguments:
  [PROJECT]  Jump to a project by index or name

Options:
  -a, --add <PATH>       Add a new project path
  -r, --remove <INDEX>   Remove a project by index
  -l, --list             List saved projects
  -h, --help             Print help
  -V, --version          Print version
```

### Examples

Add a project:

```sh
teleproj --add ~/Projects/blog-app
teleproj --add ~/Projects/todo-cli
teleproj --add ~/Projects/game-engine
```

List saved projects:

```sh
tp --list
# Saved projects:
# 0: blog-app (/home/user/Projects/blog-app)
# 1: todo-cli (/home/user/Projects/todo-cli)
# 2: game-engine (/home/user/Projects/game-engine)
```

Jump by index:

```sh
tp 1
# → cd into ~/Projects/todo-cli
```

Jump by name:

```sh
tp blog
# → cd into ~/Projects/blog-app
```

Remove a project:

```sh
teleproj --remove 2
```

---

## Config File

Projects are stored in:

```
~/.teleproj.toml
```

Example:

```toml
paths = [
  "/home/user/Projects/blog-app",
  "/home/user/Projects/todo-cli",
  "/home/user/Projects/game-engine"
]
```

---
    

## License

Teleproj is licensed under the MIT License. See `LICENSE` for details.

