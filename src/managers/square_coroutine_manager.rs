use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

pub struct SquareCoroutineManager {
    // State of the Manager
    pub state: Vec<i32>,
    // Chunk Size to divide the State
    pub chunk_size: usize,
    // Square calculate delay
    pub delay: u64,
}

impl SquareCoroutineManager {
    pub fn new(state: Vec<i32>, chunk_size: usize, delay: u64) -> Result<Self, &'static str> {
        if chunk_size < 1 {
            return Err("Chunk size must be at least 1");
        }
        Ok(Self {
            state,
            chunk_size,
            delay,
        })
    }

    pub async fn execute(&self) -> Vec<i32> {
        let chunks: Vec<Vec<i32>> = self
            .state
            .chunks(self.chunk_size) // divide the vector in partitions
            .map(|c| c.to_vec())// chunk to vec
            .collect(); // collect all vectors

        stream::iter(chunks) // stream of chunks started
            .map(|chunk| async move { // enable async process and move the process to the closure
                sleep(Duration::from_millis(self.delay)).await; // simulate
                chunk.into_iter().map(|x| x * x).collect::<Vec<i32>>() // calculate the square
            })
            .buffer_unordered(4) // set the concurrency limit
            .flat_map(|chunk: Vec<i32>| stream::iter(chunk)) // flat values
            .collect() // collect all values
            .await
    }
}
