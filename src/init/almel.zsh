almel_preexec() {
}

almel_precmd() {
    STATUS=$?
    NUM_JOBS=$#jobstates
    PROMPT="$(almel prompt zsh -s$STATUS -j$NUM_JOBS)"
}

almel_setup() {
    autoload -Uz add-zsh-hook

    add-zsh-hook precmd almel_precmd
    add-zsh-hook preexec almel_preexec
}

almel_setup
