# Almel

[![](https://badgen.net/crates/v/almel)](https://crates.io/crates/almel)

A ZSH theme inspired by [agnoster-zsh-theme](https://github.com/agnoster/agnoster-zsh-theme), written in Rust.

![](docs/almel.png)

## Installation

### From source

```sh
cargo install almel
```

### From precompiled binary

I provide precompiled binary for mac and for Windows in the [releases](releases).

### Bash

Add the following to your `.bashrc`.

```bash
eval "$(almel init bash)"
```

### Zsh

Add the following to your `.zshrc`.

```zsh
eval "$(almel init zsh)"
```

### Fish

Add the following to your `~/.config/fish/config.fish`:

```fish
almel init zsh | source
```

## Configuration

Almel looks for the configuration file at the following paths:

1. `$ALMEL_CONFIG_FILE`
2. `$XDG_CONFIG_HOME/almel/almel.yml`
3. `$HOME/.config/almel/almel.yml`
