almel_precmd() {
    STATUS=$?
    NUM_JOBS="$(jobs | wc -l)"
    PS1="$(almel prompt bash -s$STATUS -j$NUM_JOBS)"
}

PROMPT_COMMAND=almel_precmd
