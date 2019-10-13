pub fn init(shell: &str) {
    match shell {
        "zsh" => println!("{}", include_str!("init.zsh")),
        _ => panic!("unknown shell: '{}'. Supported shells: zsh", shell),
    };
}
