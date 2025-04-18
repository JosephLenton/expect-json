<div align="center">
  <h1>
    Expect Json Macros
  </h1>

  <h3>
    Macros for Expect Json,<br/>
    go look at that project.
  </h3>

  [![crate](https://img.shields.io/crates/v/expect-json-macros.svg)](https://crates.io/crates/expect-json-macros)
  [![docs](https://docs.rs/expect-json-macros/badge.svg)](https://docs.rs/expect-json-macros)

  <br/>
</div>

**This is still a work in progress.** Come back later when more is done!

Declare your expectations in your Json:

```rust
use expect_json::expect;

server
    .post(&"/user")
    .await
    .assert_json(&json!({
        "name": "Joe",
        "age": expect.in_range(20..=30),
        "timestamp": expect.iso_date_time(),
        "ids": expect.contains(&[1, 2, 3, 4]),
        "comments": [
            {
                "timestamp": expect.iso_date_time().greater_than("2025-01-01"),
                "content": "Hello!"
            }
        ]
    }));
```

# Supports

 * `expect.contains("a string")`
 * `expect.contains([1, "2", 3.3, true, false, {}])`
