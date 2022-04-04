# `nil`

🦀🚀🔥 A blazingly fast and memory-efficient implementation of `if err != nil` 🔥🚀🦀

## Example

```rust
use nil::prelude::*;

#[allow(clippy::needless_return)] // Ignore this line.
fn thing() -> (isize, error) {
    return (1, nil());
}

fn main() {
    let (n, err) = thing();
    if err != nil() {
        println!("Oh no!");
    }

    println!("Got {n}.");
}
```
