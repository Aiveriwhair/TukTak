pub mod concurrence;
pub mod test;
pub mod tuktak;

#[async_std::main]
async fn main() {
    test::test::test_process_all_async(100).await;
}
