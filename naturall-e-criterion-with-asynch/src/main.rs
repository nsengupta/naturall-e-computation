use naturall_e_including_asynch::inner::calculate_e_asynchronously_internal;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let p = calculate_e_asynchronously_internal(10_000).await;

    println!("epsilon = {}",p);
}
