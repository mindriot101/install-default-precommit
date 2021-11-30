use eyre::Result;
use structopt::StructOpt;

#[derive(Debug)]
enum Language {
    Python,
    Rust,
    Go,
}

impl std::str::FromStr for Language {
    type Err = eyre::Report;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "python" => Ok(Language::Python),
            "rust" => Ok(Language::Rust),
            "go" => Ok(Language::Go),
            other => Err(eyre::eyre!("unsupported language: {}", other)),
        }
    }
}

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(short, long)]
    language: Language,

    #[structopt(short, long)]
    force: bool,
}

fn main() {
    let opts = Opts::from_args();
    dbg!(opts);
}
