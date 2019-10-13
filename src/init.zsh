unsetopt prompt_subst;

preexec() {
}

precmd_almel() {
    PROMPT="$(almel prompt zsh --exit-status="$?")"
}

for s in "${precmd_functions[@]}"; do
    if [ "$s" = precmd_almel ]; then
        return
    fi
done

precmd_functions+=(precmd_almel)
