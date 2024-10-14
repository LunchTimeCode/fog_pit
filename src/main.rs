mod commands;
mod out;

#[tokio::main]
async fn main() {
    let res = tokio::task::spawn_blocking(commands::figure)
        .await
        .expect("async comp not working")
        .await;

    res.swallow();
}
