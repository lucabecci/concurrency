mod constructor_tests {
    use crate::managers::square_event_based_manager::SquareEventBasedManager;

    #[tokio::test]
    async fn test_new_valid_parameters() {
        let result = SquareEventBasedManager::new(vec![1, 2, 3], 2);
        assert!(result.is_ok());

        let manager = result.unwrap();
        assert_eq!(manager.state, vec![1, 2, 3]);
        assert_eq!(manager.chunk_size, 2);
    }

    #[tokio::test]
    async fn test_new_invalid_chunk_size() {
        let result = SquareEventBasedManager::new(vec![1, 2, 3], 0);
        assert!(result.is_err());
    }
}
