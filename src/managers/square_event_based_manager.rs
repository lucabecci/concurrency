use std::slice::{Chunks};
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;
use tokio::task;

pub struct SquareEventBasedManager {
    // State of the Manager
    pub state: Vec<i32>,
    // Chunk Size to divide the State
    pub chunk_size: usize,
    // Square calculate delay
    pub delay: u64,
}

impl SquareEventBasedManager {
    pub fn new(list: Vec<i32>, chunk_size: usize, delay: u64) -> Result<Self, &'static str> {
        if chunk_size < 1 {
            return Err("Chunk size must be at least 1");
        }
        Ok(Self {
            state: list,
            chunk_size,
            delay,
        })
    }

    pub async fn execute(&self) -> Vec<i32> {
        println!("List to process: {:?}", self.state);
        println!("Chunk size to use: {}", self.chunk_size);
        // Event channel to save the results
        // Use 32 to reduce the backpressure of the channel
        let (sender, mut receiver) = mpsc::channel(32);

        // Parts of the state
        let chunks = self.divide();

        // Sender handlers
        let mut handles = Vec::new();

        let mut iteration = 0;
        for chunk_values in chunks {
            println!("Values: {:?}", chunk_values);
            // Clone the sender for the task::spawn context
            let sender_clone = sender.clone();
            let chunk = chunk_values.to_vec();
            let delay = self.delay;

            // Handler of the sender context
            let handle = task::spawn(async move {
                let result: Vec<i32> = Self::calculate_square(chunk, delay).await;
                if sender_clone.send(result).await.is_err() {
                    eprint!("Error sending results to channel");
                } else {
                    println!("Iteration processed {}", iteration)
                }
            });
            handles.push(handle);
            iteration += 1;
        }
        // Drop the Sender after build all channels
        drop(sender);

        // Wait all tasks
        for handle in handles {
            handle.await.unwrap();
        }

        let mut completed_results: Vec<i32> = Vec::new();
        while let Some(result) = receiver.recv().await {
            completed_results.extend(result);
        }
        completed_results
    }

   pub  async fn calculate_square(chunk: Vec<i32>, delay: u64) -> Vec<i32> {
        // Wait 10000 ms for the example
        sleep(Duration::from_millis(delay)).await;
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
