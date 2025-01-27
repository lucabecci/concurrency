mod square_event_based_manager_test {
    use crate::managers::square_event_based_manager::SquareEventBasedManager;
    mod constructor_tests {
        use super::*;
        #[tokio::test]
        async fn test_new_valid_parameters() {
            let result = SquareEventBasedManager::new(vec![1, 2, 3], 2, 100);
            assert!(result.is_ok());

            let manager = result.unwrap();
            assert_eq!(manager.state, vec![1, 2, 3]);
            assert_eq!(manager.chunk_size, 2);
        }

        #[tokio::test]
        async fn test_new_invalid_chunk_size() {
            let result = SquareEventBasedManager::new(vec![1, 2, 3], 0, 100);
            assert!(result.is_err());
        }
    }

    mod calculate_tests {
        use crate::managers::square_event_based_manager::SquareEventBasedManager;

        #[tokio::test]
        async fn test_calculate_single_chunk() {
            let result = SquareEventBasedManager::calculate_square(vec![1, 2, 3], 100).await;
            assert_eq!(result.len(), 3);
            assert_eq!(result, &[1, 4, 9]);
        }

        #[tokio::test]
        async fn test_calculate_empty_chunk() {
            let result = SquareEventBasedManager::calculate_square(vec![], 100).await;
            assert_eq!(result.len(), 0);
            assert!(result.is_empty());
        }

        #[tokio::test]
        async fn test_calculate_large_vector() {
            let result =
                SquareEventBasedManager::calculate_square(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 100)
                    .await;
            assert_eq!(result.len(), 10);
            assert_eq!(result, &[1, 4, 9, 16, 25, 36, 49, 64, 81, 100]);
        }
    }

    mod execute_tests {
        use super::*;
        use std::time::Instant;

        #[tokio::test]
        async fn test_execute_basic_processing() {
            let manager = SquareEventBasedManager::new(vec![1, 2, 3, 4], 2, 500).unwrap();
            let mut result = manager.execute().await;
            result.sort();
            assert_eq!(vec![1, 4, 9, 16], result)
        }

        #[tokio::test]
        async fn test_execute_empty_vec() {
            let manager = SquareEventBasedManager::new(vec![], 3, 500).unwrap();
            let result = manager.execute().await;
            assert_eq!(result.len(), 0);
            assert_eq!(result, vec![]);
        }

        #[tokio::test]
        async fn test_execute_parallel_processing() {
            let manager = SquareEventBasedManager::new(vec![1, 2, 3, 4], 2, 500).unwrap();
            let start = Instant::now();
            manager.execute().await;
            let duration = start.elapsed();
            assert!(
                duration.as_millis() < 1000,
                "Processing took too long, might not be parallel"
            );
        }
    }
}
