almel::preexec() {
    ALMEL_START="$EPOCHREALTIME"
}

almel::precmd() {
    local exit_status="$?"
    local jobs="$#jobstates"
    local end="$EPOCHREALTIME"
    local duration="$(($end - ${ALMEL_START:-$end}))"
    PROMPT="$(almel prompt zsh --exit-status="$exit_status" --num-jobs="$jobs" --duration="$duration")"
    unset ALMEL_START
}

almel::setup() {
    autoload -Uz add-zsh-hook

    add-zsh-hook precmd almel::precmd
    add-zsh-hook preexec almel::preexec
}

almel::setup
