almel_preexec() {
}

almel_precmd() {
    PROMPT="$(almel prompt zsh -s$? -j$#jobstetes)"
}

almel_setup() {
    autoload -Uz add-zsh-hook

    add-zsh-hook precmd almel_precmd
    add-zsh-hook preexec almel_preexec
}

almel_setup
