almel::preexec() {
    ALMEL_START="$EPOCHREALTIME"
}

almel::async::callback() {
    PROMPT="$3"
    zle .reset-prompt
    async_stop_worker almel_async_worker
}

almel::async::prompt() {
    local exit_status="$1"
    local jobs="$2"
    local duration="$3"
    almel prompt zsh --exit-status="$exit_status" --num-jobs="$jobs" --duration="$duration"
}

almel::async(){
    async_init
    async_start_worker almel_async_worker -n
    async_register_callback almel_async_worker almel::async::callback
    async_job almel_async_worker almel::async::prompt "$@"
}

almel::precmd() {
    local exit_status="$?"
    local jobs="$#jobstates"
    local end="$EPOCHREALTIME"
    local duration="$(($end - ${ALMEL_START:-$end}))"
    if (( ${+functions[async_init]} )); then
        PROMPT="$(almel prompt zsh --exit-status="$exit_status" --num-jobs="$jobs" --duration="$duration" --no-git)"
        almel::async "$exit_status" "$jobs" "$duration"
    else
        PROMPT="$(almel prompt zsh --exit-status="$exit_status" --num-jobs="$jobs" --duration="$duration")"
    fi
    unset ALMEL_START
}

almel::setup() {
    autoload -Uz add-zsh-hook

    add-zsh-hook precmd almel::precmd
    add-zsh-hook preexec almel::preexec
}

almel::setup
