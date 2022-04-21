# Generate-Random

Generate random data.

## Example

```rust
use generate_random::GenerateRandom;

#[derive(GenerateRandom)]
enum MyEnum {
    A,
    C(bool),
    B {
        x: u8,
    },
    // Providing a weight allows changing the probabilities.
    // This variant is now twice as likely to be generated as the others.
    #[weight(2)]
    D,
}

let mut rng = rand::thread_rng();
let my_value = MyEnum::generate_random(&mut rng);
```

## Documentation

[Documentation](https://lib.rs/crates/generate-random)
