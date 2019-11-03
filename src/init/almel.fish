function fish_prompt
    almel prompt fish -s$status -j(count (jobs -p)) -d(math $CMD_DURATION / 1000)
end
