unsetopt prompt_subst;

preexec() {
}

precmd_almel() {
    PROMPT="$(exit_status="$?" jobs=$(jobs | wc -l) almel prompt zsh)"
}

for s in "${precmd_functions[@]}"; do
    if [ "$s" = precmd_almel ]; then
        return
    fi
done

precmd_functions+=(precmd_almel)
