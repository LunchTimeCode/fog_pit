use clap::{Parser, Subcommand};

use crate::terminal_out::{green_fog, red_fog, Foggy};

pub async fn figure() -> Foggy {
    let cli = Cli::parse();

    let result: anyhow::Result<String> = match cli.command {
        None => Ok("try fog --help for information on how to use fog".into()),
        Some(_) => Ok("Nothing yet".into()),
    };

    match result {
        Ok(o) => green_fog(o),
        Err(err) => red_fog(err.to_string()),
    }
}

/// dreamy cli
#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "fog")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
#[command(rename_all = "snake_case")]
enum Commands {
    /// [STABLE] print markdown doc of fog to std out
    Markdown,
    /// [Preview] does nothing for now
    Pit,
}
