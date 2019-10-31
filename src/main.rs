mod opt;
mod shell;

use opt::Opt;

fn main() {
    let opt = Opt::parse();
    println!("{:#?}", opt);
}
