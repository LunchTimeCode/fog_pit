use colored::Colorize;
mod commands;
mod terminal_out;

#[tokio::main]
async fn main() {
    let res = tokio::task::spawn_blocking(commands::figure)
        .await
        .expect("async comp not working")
        .await;

    match res {
        terminal_out::Foggy::Green(g) => {
            println!("{g}");
        }
        terminal_out::Foggy::Red(r) => {
            println!("{r}");
            std::process::exit(1)
        }
        terminal_out::Foggy::Yellow(y) => {
            println!("{y}");
        }
    }
}
