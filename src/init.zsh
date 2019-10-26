unsetopt prompt_subst;

almel_preexec() {
}

almel_precmd() {
    PROMPT="$(EXIT_STATUS="$?" JOBS=$(jobs) almel prompt zsh)"
}

almel_setup() {
    autoload -Uz add-zsh-hook

    add-zsh-hook precmd almel_precmd
    add-zsh-hook preexec almel_preexec
}

almel_setup
