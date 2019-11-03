almel_preexec() {
    ALMEL_START="$EPOCHREALTIME"
}

almel_precmd() {
    STATUS="$?"
    NUM_JOBS="$#jobstates"
    END="$EPOCHREALTIME"
    DURATION="$(($END - ${ALMEL_START:-$END}))"
    PROMPT="$(almel prompt zsh -s"$STATUS" -j"$NUM_JOBS" -d"$DURATION")"
    unset ALMEL_START
}

almel_setup() {
    autoload -Uz add-zsh-hook

    add-zsh-hook precmd almel_precmd
    add-zsh-hook preexec almel_preexec
}

almel_setup
