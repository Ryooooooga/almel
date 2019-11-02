almel_preexec() {
}

almel_precmd() {
    STATUS=$?
    NUM_JOBS="$(jobs | wc -l)"
    PS1="$(almel prompt bash -s$STATUS -j$NUM_JOBS)"
}

almel_setup() {
    PROMPT_COMMAND=almel_precmd
}

almel_setup
