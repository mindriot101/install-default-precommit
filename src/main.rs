use eyre::{Result, WrapErr};
use std::path::PathBuf;
use structopt::StructOpt;

mod config;

#[derive(Debug, Clone, Copy)]
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

fn find_project_root() -> Result<PathBuf> {
    Ok(PathBuf::from("."))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let opts = Opts::from_args();

    let project_root = find_project_root().wrap_err("finding project root")?;
    let output_name = project_root.join(".pre-commit-config.yaml");
    if output_name.is_file() && !opts.force {
        eyre::bail!("file {:?} exists, not overwriting", output_name);
    }

    let config = config::Config::for_language(opts.language);
    let mut outfile = std::fs::File::create(&output_name).wrap_err("creating output file")?;
    serde_yaml::to_writer(&mut outfile, &config).wrap_err("serializing configuration")?;

    Ok(())
}
