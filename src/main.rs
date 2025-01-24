
mod managers;

use managers::square_event_based_manager::SquareEventBasedManager;

#[tokio::main]
async fn main() {
    println!("Starting event based manager");
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let manager = SquareEventBasedManager::new(vector, 3).unwrap();

    let results = manager.execute().await;
    println!("Results: {:?}", results);
}
