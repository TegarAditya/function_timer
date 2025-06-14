# Function Timer Macro
A simple procedural attribute macro for Rust that times the execution of a function and logs the result using the tracing crate.

This macro is designed to be a convenient utility for performance monitoring during development. It correctly handles both synchronous and async functions.

## Features
- Easy to apply with a #[time] attribute.
- Works seamlessly with both async and standard functions.
- Logs the function name and execution time in milliseconds.
- Uses the tracing crate for logging, integrating well with existing logging infrastructure.

## Prerequisites
Your main application must use the tracing crate for the log output to be visible.

## Usage
#### 1. Add the Crate as a Local Dependency
In the Cargo.toml of your main application, add this crate using a path dependency:
```toml
[dependencies]
# ... other dependencies
function_timer = { git = "https://github.com/TegarAditya/function_timer/" }
```

#### 2. Import the Macro
In the Rust file where you want to use the timer, import the macro:
```rust
use function_timer::time;
```

#### 3. Apply the Attribute
Place the `#[time]` attribute directly above any function you wish to time.

**Example with an async function:**
```rust
use function_timer::time;
use std::time::Duration;
use tokio;

#[time]
async fn my_async_function() {
    // Simulate some async work
    tokio::time::sleep(Duration::from_millis(100)).await;
}
```

**Example with a synchronous function:**
```rust
use function_timer::time;
use std::thread;
use std::time::Duration;

#[time]
fn my_sync_function() {
    // Simulate some work
    thread::sleep(Duration::from_millis(50));
}
```

## Example Log Output
When a timed function is executed, it will produce a log message similar to this:
```bash
--- Function 'my_async_function' executed in 100ms
--- Function 'my_sync_function' executed in 50ms
```
