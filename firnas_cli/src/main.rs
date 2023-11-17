pub mod firnas;
use clap::Parser;
use firnas::Firnas;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Firnas,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Firnas::Compile { path, extentions } => Firnas::handle_file(path.to_string(), &extentions),
    }
}
