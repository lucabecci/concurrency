pub struct EventBasedManager {
    // State of the Manager
    pub state: Vec<i32>,
    // Chunk Size to divide the State
    pub chunk_size: i16,
}

impl EventBasedManager {
    pub fn new(list: Vec<i32>, chunk_size: i16) -> EventBasedManager {
        EventBasedManager::initialize(list, chunk_size)
    }
    fn initialize(list: Vec<i32>, chunk_size: i16) -> Self {
        Self { state: list, chunk_size, }
    }
    pub fn execute(&self) {
        println!("List to process: {:?}", self.state);
        println!("Chunk size to use: {}", self.chunk_size);
    }
}
