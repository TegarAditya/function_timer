# Function Timer Macro
A simple procedural attribute macro for Rust that times the execution of a function and logs the result using the tracing crate.

This macro is designed to be a convenient utility for performance monitoring during development. It correctly handles both synchronous and async functions.

## Features
- Easy to apply with a `#[time]` attribute.
- Works seamlessly with both async and standard functions.
- Logs the function name and execution time in milliseconds.
- Uses the tracing crate for logging, integrating well with existing logging infrastructure.


## How It Works & Prerequisites

The `tracing` ecosystem is designed with a two-part architecture:

1.  **Instrumentation (`tracing`)**: Code that generates log events. Our `#[time]` macro does this by calling `tracing::info!`.
2.  **Subscription (`tracing-subscriber`)**: A listener that processes these events and decides what to do with them (e.g., print them to the console).

This `function_timer` crate only handles the first part (instrumentation). For the log messages to be visible, **your main application must handle the second part by setting up a subscriber.**

If you don't initialize a subscriber in your `main.rs`, the logs from `#[time]` will be generated but then immediately discarded, and you will see no output.

A typical subscriber setup in your `main.rs` looks like this:

```rust
// in main.rs
fn main() {
    // This line sets up a subscriber that prints logs to the console.
    tracing_subscriber::fmt().init();

    // ... rest of your application ...
}
```

## Usage
#### 1. Add the Crate as a Dependency
In the Cargo.toml of your main application, add this crate using a 'git' dependency:
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
