function fish_prompt
    almel prompt fish -s$status -j(count (jobs -p))
end
