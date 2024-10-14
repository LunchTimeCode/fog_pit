use clap::{CommandFactory, Parser, Subcommand};
use clap_help::Printer;

use crate::out::{blue_fog, explain_fog, green_fog, white_fog, Foggy};

pub async fn figure() -> Foggy {
    let cli = Cli::parse();
    match cli.command {
        None => {
            Printer::new(Cli::command()).print_help();
            blue_fog("Visit https://github.com/LunchTimeCode/fog_pit for more")
        }
        Some(commands) => match commands {
            Commands::Markdown => white_fog(clap_markdown::help_markdown::<Cli>()),
            Commands::Explain => {
                explain_fog();
                white_fog("This should never happen")
            }
            Commands::Pit => green_fog("Nothing yet"),
        },
    }
}

/// fog cli
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about,
    name = "fog",
    disable_help_flag = true
)]
struct Cli {
    /// Print help
    #[arg(long)]
    help: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "snake_case")]
enum Commands {
    /// [STABLE] print markdown doc of fog to std out
    Markdown,
    Explain,
    /// [Preview] does nothing for now
    Pit,
}
