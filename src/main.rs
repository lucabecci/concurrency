
mod managers;

use managers::event_based_manager::EventBasedManager;

fn main() {
    println!("Starting event based manager");
    let manager = EventBasedManager::new(
        vec![123], 3
    );

    manager.execute();
}
