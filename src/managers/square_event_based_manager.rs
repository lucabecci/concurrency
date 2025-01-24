use std::slice::{Chunks};
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;
use tokio::task;

pub struct SquareEventBasedManager {
    // State of the Manager
    pub state: Vec<i32>,
    // Chunk Size to divide the State
    pub chunk_size: usize,
}

impl SquareEventBasedManager {
    pub fn new(list: Vec<i32>, chunk_size: usize) -> Result<Self, &'static str> {
        if(chunk_size < 1) {
            return Err("Chunk size must be at least 1");
        }
        Ok(Self {
            state: list,
            chunk_size
        })
    }

    pub async fn execute(&self) -> Vec<i32> {
        println!("List to process: {:?}", self.state);
        println!("Chunk size to use: {}", self.chunk_size);
        // Event channel to save the results
        // Use 32 to reduce the backpressure of the channel
        let (tx, mut rx) = mpsc::channel(32);

        // Parts of the state
        let chunks = self.divide();

        let mut iteration = 0;
        for chunk_values in chunks {
            println!("Values: {:?}", chunk_values);
            let tx = tx.clone();
            let chunk = chunk_values.to_vec();

            task::spawn(async move {
                let result: Vec<i32> = Self::calculate_chunk_squares(chunk).await;
                if tx.send(result).await.is_err() {
                    eprint!("Error sending results to channel");
                } else {
                    println!("Chunk processed {}", iteration)
                }
            });
            iteration += 1;
        }

        let mut completed_results: Vec<i32> = Vec::new();
        while let Some(result) = rx.recv().await {
            completed_results.extend(result);
        }
        completed_results
    }

    async fn calculate_chunk_squares(chunk: Vec<i32>) -> Vec<i32> {
        // Wait 10000 ms for the example
        sleep(Duration::from_millis(1)).await;
        // Use into_iter to get the original memory values of chunk
        chunk.into_iter().map(|x| x * x).collect()
    }


    /**
    * Divide the state in multiple parts with the chunk() and chunk_size
    */
    fn divide<'a>(&self) -> Chunks<i32> {
        self.state.chunks(self.chunk_size)
    }
}
