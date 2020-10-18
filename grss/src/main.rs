use structopt::StructOpt;

#[derive(Debug)]
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf
}

fn main() {
    let args = Cli::from_args();
    println!("{:#?}", args)
}
