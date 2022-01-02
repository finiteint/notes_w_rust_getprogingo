#[async_std::main]
async fn main() {
    exploring_concurrency_with_dystopia().await;
}

pub async fn exploring_concurrency_with_dystopia() {
    u07::asynced::waiting_on_workers_with_join_handles().await;
    u07::asynced::waiting_on_workers_with_channels().await;
    u07::asynced::facts_pipeline().await;
}
