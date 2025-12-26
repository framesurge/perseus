# Testing

Perseus apps benefit from multiple testing strategies: unit tests for logic, integration tests for state generation, and end-to-end (E2E) tests for full user flows.

## End-to-End Testing

E2E tests run a real browser against your app, simulating user interactions.

### Writing E2E Tests

Place tests in a `tests/` directory:

```rust
// tests/main.rs
use fantoccini::Client;
use perseus::wait_for_checkpoint;

#[perseus::test]
async fn test_homepage(client: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    // Navigate to the app
    client.goto("http://localhost:8080").await?;

    // Wait for Perseus to initialize
    wait_for_checkpoint!("page_interactive", 0, client);

    // Check the page content
    let title = client.find(fantoccini::Locator::Css("h1")).await?;
    assert_eq!(title.text().await?, "Welcome");

    Ok(())
}

#[perseus::test]
async fn test_navigation(client: &mut Client) -> Result<(), fantoccini::error::CmdError> {
    client.goto("http://localhost:8080").await?;
    wait_for_checkpoint!("page_interactive", 0, client);

    // Click a link
    let link = client.find(fantoccini::Locator::Css("a[href='/about']")).await?;
    link.click().await?;

    // Wait for navigation to complete
    wait_for_checkpoint!("page_interactive", 1, client);

    // Verify we're on the about page
    let heading = client.find(fantoccini::Locator::Css("h1")).await?;
    assert_eq!(heading.text().await?, "About Us");

    Ok(())
}
```

### Running Tests

```bash
# Start a WebDriver (in a separate terminal)
geckodriver    # For Firefox
chromedriver   # For Chrome

# Run tests
perseus test
```

The `perseus test` command:
1. Builds your app in test mode
2. Starts a test server
3. Runs all tests (unit, integration, and E2E)
4. Reports results

### Debugging Tests

Show the browser during tests:

```bash
perseus test --show-browser
```

This disables headless mode so you can see what's happening.

## Checkpoints

Perseus emits checkpoints at key moments during testing. Use `wait_for_checkpoint!` to synchronize tests.

### Available Checkpoints

| Checkpoint | Meaning |
|------------|---------|
| `begin` | Perseus has initialized |
| `page_interactive` | Page is fully hydrated and interactive |
| `error` | An error occurred |
| `not_found` | Page not found (also emits `error`) |

### Checkpoint Indices

The index (second argument) counts occurrences:

```rust
// Wait for the first page to be interactive
wait_for_checkpoint!("page_interactive", 0, client);

// Navigate...

// Wait for the second page to be interactive
wait_for_checkpoint!("page_interactive", 1, client);
```

Checkpoints persist across navigations but reset on page refresh.

### Custom Checkpoints

Define your own checkpoints for complex flows:

```rust
use perseus::checkpoint;

fn my_component() -> View {
    // Emit a custom checkpoint
    checkpoint("custom_data_loaded");

    view! { p { "Data loaded!" } }
}

// In your test
wait_for_checkpoint!("custom_data_loaded", 0, client);
```

Custom checkpoint names must:
- Start with "custom"
- Contain no hyphens (use underscores)

## Unit Testing

Test pure functions normally:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date() {
        assert_eq!(format_date(2024, 1, 15), "January 15, 2024");
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com"));
        assert!(!validate_email("invalid"));
    }
}
```

Run with:

```bash
cargo test
```

## Testing State Generation

Test your state generation functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_state() {
        let info = StateGeneratorInfo {
            path: "my-page".to_string(),
            locale: "en-US".to_string(),
            extra: ().into(),
        };

        let state = get_build_state(info).await;
        assert_eq!(state.title, "My Page");
    }
}
```

## WebDriver Setup

### Firefox (geckodriver)

1. Install: Download from [Mozilla's releases](https://github.com/mozilla/geckodriver/releases)
2. Run: `geckodriver` (defaults to port 4444)

### Chrome (chromedriver)

1. Install: Download from [Chrome for Testing](https://googlechromelabs.github.io/chrome-for-testing/)
2. Run: `chromedriver --port=4444`

### CI Setup

```yaml
# GitHub Actions example
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: browser-actions/setup-geckodriver@latest
      - run: geckodriver &
      - run: perseus test
```

## Tips

1. **Wait for checkpoints** - Don't assume timing; use checkpoints
2. **Test critical paths** - Focus on user journeys
3. **Clean state** - Each test should be independent
4. **Debug visually** - Use `--show-browser` when tests fail
5. **Add delays for debugging** - `std::thread::sleep()` to observe state

## Single-Threaded Limitation

E2E tests currently run single-threaded due to WebDriver limitations. This makes them slower but ensures reliability.

## Related

- [Debugging](/docs/fundamentals/debugging)
- [Checkpoints API](https://docs.rs/perseus/latest/perseus/fn.checkpoint.html)
- [Fantoccini Documentation](https://docs.rs/fantoccini)
