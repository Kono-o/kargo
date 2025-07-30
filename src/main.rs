mod func;
mod kargo;

#[tokio::main]
async fn main() {
    kargo::run().await;
}
