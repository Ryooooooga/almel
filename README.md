# Almel

<a href="https://crates.io/crates/almel"><img src="https://badgen.net/crates/v/almel"></a>

A ZSH theme inspired by [agnoster-zsh-theme](https://github.com/agnoster/agnoster-zsh-theme), written in Rust.

![](docs/almel.png)

## Installation

### From source

```sh
cargo install almel
```

### From precompiled binary

I provide precompiled binary in [releases](releases).

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
almel init fish | source
```

## Configuration

Almel looks for the configuration file at the following paths:

1. `$ALMEL_CONFIG_FILE`
2. `$XDG_CONFIG_HOME/almel/almel.yml`
3. `$HOME/.config/almel/almel.yml`
