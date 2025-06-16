use function_timer::time;
use temp_env;

#[time]
fn timed_sync_function() -> i32 {
    42
}

#[time]
async fn timed_async_function() -> String {
    "hello".to_string()
}

#[test]
fn test_sync_function_returns_correct_value() {
    assert_eq!(timed_sync_function(), 42);
}

#[tokio::test]
async fn test_async_function_returns_correct_value() {
    assert_eq!(timed_async_function().await, "hello");
}

#[tokio::test]
async fn test_timer_can_be_disabled() {
    temp_env::with_var("ENABLE_FUNCTION_TIMER", Some("false"), || async {
        assert_eq!(timed_sync_function(), 42);
        assert_eq!(timed_async_function().await, "hello");
    })
    .await;
}
